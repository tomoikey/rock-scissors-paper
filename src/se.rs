// pub struct Se {
//     audio_device: AudioDevice<SquareWave>,
// }
//
// impl Se {
//     pub fn play(&self) {
//         thread::scope(|s| {
//             s.spawn(|| {
//                 self.audio_device.resume();
//                 thread::sleep(std::time::Duration::from_millis(100));
//                 self.audio_device.pause();
//             });
//         });
//     }
// }
//
// pub struct SquareWave {
//     phase_inc: f32,
//     phase: f32,
//     volume: f32,
// }
//
// impl AudioCallback for SquareWave {
//     type Channel = f32;
//
//     fn callback(&mut self, out: &mut [f32]) {
//         // Generate a square wave
//         for x in out.iter_mut() {
//             *x = if self.phase <= 0.5 {
//                 self.volume
//             } else {
//                 -self.volume
//             };
//             self.phase = (self.phase + self.phase_inc) % 1.0;
//         }
//     }
// }
