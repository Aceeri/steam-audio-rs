use std::fmt;

#[derive(Clone)]
pub struct DeinterleavedFrame {
    pub current_frame: Vec<Vec<f32>>,
    ptrs: Vec<*mut f32>,
    sample_rate: u32,
    channel_offset: u16,
    frame_offset: usize,
}

impl fmt::Debug for DeinterleavedFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use rodio::Source;
        formatter
            .debug_struct("DeinterleavedFrame")
            .field("channels", &self.channels())
            .field("sample_rate", &self.sample_rate())
            .field("frame_size", &self.frame_size())
            .finish()
    }
}

impl DeinterleavedFrame {
    pub fn new(frame_size: usize, channels: u16, sample_rate: u32) -> Self {
        let mut frame = Self {
            current_frame: vec![vec![0.0; frame_size]; channels as usize],
            ptrs: Vec::new(),
            sample_rate: sample_rate,
            channel_offset: 0,
            frame_offset: 0,
        };

        frame.ptrs = frame
            .current_frame
            .iter()
            .map(|d| d.as_ptr() as *mut _)
            .collect();
        frame
    }

    pub fn from_source<S, I>(frame_size: usize, source: &mut S) -> Self
    where
        I: rodio::Sample,
        S: rodio::Source + Iterator<Item = I>,
    {
        let mut frame = Self::new(frame_size, source.channels(), source.sample_rate());
        frame.push_source(source);
        frame
    }

    pub fn push_source<S, I>(&mut self, source: &mut S) -> bool
    where
        I: rodio::Sample,
        S: rodio::Source + Iterator<Item = I>,
    {
        let mut channel = 0;
        let mut frame = 0;

        while let Some(sample) = source.next() {
            self.current_frame[channel][frame] = sample.to_f32();

            if channel as u16 >= self.channels() - 1 {
                channel = 0;

                if frame >= self.frame_size() - 1 {
                    return true;
                }

                frame += 1;
            } else {
                channel += 1;
            }
        }

        false
    }

    pub fn frame_size(&self) -> usize {
        self.current_frame.get(0).map(|d| d.len()).unwrap_or(0)
    }

    pub fn channels(&self) -> u16 {
        self.current_frame.len() as u16
    }

    pub unsafe fn ptrs(&mut self) -> *mut *mut f32 {
        self.ptrs.as_mut_ptr()
    }
}

impl rodio::Source for DeinterleavedFrame {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.frame_size())
    }

    fn channels(&self) -> u16 {
        self.current_frame.len() as u16
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for DeinterleavedFrame {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self
            .current_frame
            .get(self.channel_offset as usize)
            .map(|channel| channel.get(self.frame_offset))
            .flatten()
            .cloned();

        if self.channels() == self.channel_offset {
            self.channel_offset = 0;
            self.frame_offset += 1;
        } else {
            self.channel_offset += 1;
        }

        sample
    }
}
