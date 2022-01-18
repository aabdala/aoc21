pub fn solution(input: &str) -> String {
    let tp = super::TransparentPaper::from(input);
    let first_fold = tp.folds[0];
    format!("{}", tp.fold(&first_fold).len())
}
