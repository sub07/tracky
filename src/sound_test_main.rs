#![feature(array_chunks)]
#![feature(let_chains)]
#![feature(associated_type_bounds)]

extern crate core;

use std::env::args;
use std::path::Path;
use std::time::Duration;

use cpal::SampleRate;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_utils::iter::zip_self::ZipSelf;
use rust_utils_macro::EnumIter;

fn plot_pcm<S: AsRef<str>>(name: S, sample: &StereoPcm) {
    plot_line_xy(name, (0..sample.data.len()).map(|i| 0.5 * i as f64 / sample.speed as f64),
                 &sample.data);
}

fn plot_line_xy<Tx, X, Ty, Y, S>(name: S, x: X, y: Y)
    where Tx: gnuplot::DataType, X: IntoIterator<Item=Tx>, Ty: gnuplot::DataType, Y: IntoIterator<Item=Ty>, S: AsRef<str> {
    use gnuplot::AxesCommon;
    let mut fg = gnuplot::Figure::new();
    fg.axes2d()
        .set_x_label("sec", &[])
        .set_title(name.as_ref(), &[])
        .set_y_label("amp", &[])
        .lines(x, y, &[]);
    fg.show_and_keep_running().unwrap();
}

struct Channel {
    pub data: StereoPcm,
    duration: Duration,
}

impl Channel {
    pub fn new(duration: Duration, sample_rate: f64) -> Channel {
        Channel {
            data: StereoPcm::from_duration(duration, sample_rate),
            duration,
        }
    }

    pub fn add(&mut self, pcm: &StereoPcm, at: Duration) {
        assert!(at < self.duration);
        let frame_index = self.data.frame_index_at_time(at.as_secs_f64());
        self.data.add_pcm(pcm, frame_index);
    }
}

struct StereoPcm {
    pub data: Vec<f32>,
    pub speed: f64,
}

impl StereoPcm {
    pub fn from_duration(duration: Duration, sample_rate: f64) -> StereoPcm {
        let data_len = (2.0 * duration.as_secs_f64() * sample_rate) as usize;
        StereoPcm {
            data: vec![0.0; data_len],
            speed: sample_rate,
        }
    }

    pub fn from_wav<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut wav = audrey::open(path)?;
        let desc = wav.description();
        assert!(desc.channel_count() == 1 || desc.channel_count() == 2);
        let data = wav.samples().map(Result::unwrap).collect::<Vec<_>>();
        Ok(Self {
            data: if desc.channel_count() == 1 { data.into_iter().zip_self(2).collect() } else { data },
            speed: desc.sample_rate() as f64,
        })
    }

    pub fn nb_frame(&self) -> usize {
        self.data.len() / 2
    }

    pub fn duration_sec(&self) -> f64 {
        (self.data.len() / 2) as f64 / self.speed as f64
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.duration_sec())
    }

    fn frame_index_at(&self, ratio: f64) -> f64 {
        (self.data.len() as f64 / 2.0 - 1.0) * ratio
    }

    pub fn sample_at(&self, ratio: f64) -> (f32, f32) {
        if ratio == 1.0 && let [.., l, r] = &self.data[..] { return (*l, *r); }
        assert!((0.0..1.0).contains(&ratio));
        let frame_index = self.frame_index_at(ratio);
        let decimal = frame_index.fract() as f32;
        let frame_index = frame_index as usize;
        let next_frame_index = frame_index + 1;

        let l1 = self.data[frame_index * 2];
        let l2 = self.data[next_frame_index * 2];

        let r1 = self.data[frame_index * 2 + 1];
        let r2 = self.data[next_frame_index * 2 + 1];

        let sampled_left = l1 * (1.0 - decimal) + l2 * decimal;
        let sampled_right = r1 * (1.0 - decimal) + r2 * decimal;
        (sampled_left, sampled_right)
    }

    pub fn add_pcm(&mut self, src: &StereoPcm, frame_index: usize) {
        let mut src_index = 0;
        let mut dest_index = frame_index * 2;

        while src_index < src.data.len() && dest_index < self.data.len() {
            self.data[dest_index] = src.data[src_index];
            src_index += 1;
            dest_index += 1;
        }
    }

    pub fn sample_at_time(&self, second: f64) -> (f32, f32) {
        assert!(second < self.duration_sec());
        self.sample_at(second / self.duration_sec())
    }

    pub fn frame_index_at_time(&self, time: f64) -> usize {
        (time * self.speed).round() as usize
    }

    pub fn energy(&self) -> f32 {
        self.data.iter().map(|s| s * s).sum::<f32>() / self.data.len() as f32
    }

    pub fn sample(&self, time: f64, freq: f64) -> (f32, f32) {
        let pitch_shifted_time = time * (freq / 440.0);
        let frame_index = self.frame_index_at_time(pitch_shifted_time);
        if frame_index > self.nb_frame() {
            (0.0, 0.0)
        } else {
            let l = self.data[frame_index * 2];
            let r = self.data[frame_index * 2 + 1];
            (l, r)
        }
    }

    pub fn pitch_shifted(&self, target_speed: f64) -> StereoPcm {
        let src_speed = self.speed as f64;
        let src_len = self.data.len() as f64;
        let ratio = target_speed / src_speed;
        let multiplier = 1.0 / ratio;
        let dest_nb_sample = self.data.len() as f64 * multiplier;
        // dbg!(src_speed, target_speed, ratio, multiplier, src.data.len(), dest_nb_sample);

        let mut dest_data = Vec::with_capacity(dest_nb_sample.round() as usize);
        let mut src_index = 0.0;

        while src_index < src_len {
            let (l, r) = self.sample_at(src_index / src_len);
            dest_data.push(l);
            dest_data.push(r);
            src_index += ratio * 2.0;
        }

        // dbg!(dest_data.len());

        StereoPcm {
            data: dest_data,
            speed: target_speed,
        }
    }
}

fn resample(src: &StereoPcm, target_sr: f64) -> StereoPcm {
    if src.speed == target_sr {
        return StereoPcm {
            data: src.data.clone(),
            speed: target_sr,
        };
    }

    let duration = src.duration_sec();

    let target_nb_sample = ((duration * target_sr as f64 + 0.5) as usize) * 2;

    let mut sec = 0f64;
    let period = 1f64 / target_sr as f64;
    let mut output_data = Vec::with_capacity(target_nb_sample);

    while sec < duration {
        let (l, r) = src.sample_at_time(sec);
        output_data.push(l);
        output_data.push(r);
        sec += period;
    }

    StereoPcm {
        speed: target_sr,
        data: output_data,
    }
}

fn pitch_shift(src: &StereoPcm, target_speed: f64) -> StereoPcm {
    let src_speed = src.speed as f64;
    let src_len = src.data.len() as f64;
    let ratio = target_speed / src_speed;
    let multiplier = 1.0 / ratio;
    let dest_nb_sample = src.data.len() as f64 * multiplier;
    // dbg!(src_speed, target_speed, ratio, multiplier, src.data.len(), dest_nb_sample);

    let mut dest_data = Vec::with_capacity(dest_nb_sample.round() as usize);
    let mut src_index = 0.0;

    while src_index < src_len {
        let (l, r) = src.sample_at(src_index / src_len);
        dest_data.push(l);
        dest_data.push(r);
        src_index += ratio * 2.0;
    }

    // dbg!(dest_data.len());

    StereoPcm {
        data: dest_data,
        speed: target_speed,
    }
}

#[derive(EnumIter, Debug)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

pub fn note_to_multiplier(semitone: i32) -> f64 {
    const NOTE_MUL: f64 = 1.0594630943593;
    NOTE_MUL.powi(semitone)
}

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();
    assert!(args.len() > 1);
    let original = StereoPcm::from_wav(&args[1])?;

    let host = cpal::default_host();

    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().unwrap();

    let SampleRate(sample_rate) = config.sample_rate();
    let channels = config.channels() as usize;

    assert_eq!(channels, 2);

    let mut channel = Channel::new(Duration::from_secs(40), sample_rate as f64);

    let mut resampled = resample(&original, sample_rate as f64);

    let mut sec = 0.5;
    let mut semitone = 0;
    for note in Note::VARIANTS {
        channel.add(&resampled.pitch_shifted(resampled.speed * note_to_multiplier(semitone)), Duration::from_secs_f64(sec));
        semitone += 1;
        sec += 0.25;
    }

    // plot_pcm("channel", &channel.data);

    let used_sample_data = channel.data.data;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let mut index = 0;

    let mut next_sample = move || {
        //     if index > used_sample_data.len() * 6 {
        //         index = 0;
        //     }
        let s = used_sample_data.get(index).cloned().unwrap_or(0.0);
        index += 1;
        s * 0.1
    };

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                frame[0] = next_sample();
                frame[1] = next_sample();
            }
        },
        err_fn,
        None,
    ).unwrap();

    stream.play().unwrap();
    std::thread::sleep(channel.duration);

    Ok(())
}
