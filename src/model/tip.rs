use std::time::Duration;

/// Tips are text that is rendered under the map. The duration of a tip indicates for how long it
/// will be rendered. If two tips are tried to be rendered at once, the one with higher priority
/// will prevail. If a tip has, for instance, 1 second left on screen, and another tip is sent with
/// lower priority and duration 2, after a second passes, it will NOT be rendered for the next
/// second. You'll need to send it again after the previous tip has expired.
#[derive(Debug, Clone)]
pub struct Tip {
    text: String,
    duration: Duration,
    priority: i32,
}

impl Tip {
    pub fn new(text: String, duration: Duration, priority: i32) -> Self {
        Tip {
            text,
            duration,
            priority,
        }
    }

    /// Returns the tip with the most priority, in case of equal priorities, the caller is
    /// returned.
    /// ```
    /// use starlib::model::tip::Tip;
    /// use std::time::Duration;
    /// let a = Tip::new(String::from("Tip a"), Duration::from_secs(1), 1);
    /// let b = Tip::new(String::from("Tip b"), Duration::from_secs(1), 1);
    /// assert_eq!(a.overlap(b).get_text(), String::from("Tip a"));
    /// ```
    pub fn overlap(self, other: Tip) -> Tip {
        if other.priority > self.priority {
            other
        } else {
            self
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Reduce the duration of the tip by a 'time' amount. If 'time' is a greater duration than
    /// that of the Tip, instead of panicking, the duration will be set to 0. Once a tip's duration
    /// reaches 0 this way, its priority is reduced to the minimum.
    pub fn ellapse(&mut self, time: Duration) {
        self.duration = if self.duration < time {
            self.priority = i32::MIN;
            Duration::ZERO
        } else {
            self.duration - time
        }
    }

    pub fn has_expired(&self) -> bool {
        self.duration == Duration::from_secs(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellapse() {
        let mut t = Tip::new(String::from("Test"), Duration::from_secs(1), 1);
        t.ellapse(Duration::from_millis(500));
        assert!(!t.has_expired());
    }

    #[test]
    fn test_ellapse_underflow() {
        let mut t = Tip::new(String::from("Test"), Duration::from_secs(1), 1);
        t.ellapse(Duration::from_secs(2));
        assert!(t.has_expired());
    }
}
