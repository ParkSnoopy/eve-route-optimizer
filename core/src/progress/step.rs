
#[derive(Clone)]
pub struct ProgressStep {
    percentage: f64,
    state: String,
}

impl ProgressStep {
    fn with_msg(progress_holder: &ProgressHolder, msg: impl AsRef<str>) -> Self {
        // downcast to float, to perform percentage calculations
        let total = progress_holder.total as f64;
        let done  = progress_holder.done  as f64;

        let done_p = 100f64 * done / total; // range=[0.0, 100.0]

        Self {
            percentage: done_p,
            state: msg.as_ref().to_string(),
        }
    }
}

impl Display for ProgressStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {:5.02} % ] {}", self.percentage, self.state)
    }
}
