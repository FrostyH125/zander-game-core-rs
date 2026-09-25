pub struct Timer {
    pub duration: f32,
    pub current_time: f32,
    is_playing: bool
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            duration,
            current_time: 0.0,
            is_playing: false
        }
    }
    
    pub fn track(&mut self, dt: f32) {
        self.is_playing = true;
        
        if self.current_time >= self.duration {
            self.current_time = self.duration;
            self.is_playing = false;
            return;
        }
        
        self.current_time += dt;
    }
    
    pub fn is_playing(&self) -> bool {
        return self.is_playing;
    }

    pub fn set_playing(&mut self) {
        self.is_playing = true;
    }
       
    pub fn is_done(&self) -> bool {
        return self.current_time >= self.duration;
    }
    
    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.is_playing = false;
    }

    pub fn progress(&self) -> f32 {
        return (self.current_time / self.duration).clamp(0.0, 1.0);
    }  
}