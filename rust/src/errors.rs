use custom_error::custom_error;

custom_error!{pub SceneIOError
    FailedToEncode{description: String} = "Failed to encode: {description}",
    FailedToDecode{description: String} = "Failed to decode: {description}",
    IOError {source: std::io::Error} = "IO Error: {source}",
    FailedToReadObj{description: String} = "Failed to read obj file: {description}",
}
