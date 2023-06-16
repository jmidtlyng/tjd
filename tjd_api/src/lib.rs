// used to track exchanges with TJD api
pub struct TjdApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub value: Option<T>
}
