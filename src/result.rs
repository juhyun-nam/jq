pub enum Results {
    Ok = 0,
    OkNullKind = -1, /* exit 0 if --exit-status is not set*/
    ErrorSystem = 2,
    ErrorCompile = 3,
    OkNoOutput = -4, /* exit 0 if --exit-status is not set*/
    ErrorUnknown = 5,
}
