use thiserror::Error;
use std::net::SocketAddr;

#[derive(Debug, Error)]
enum CancellationError {

    // AuthenticationFailure indicates an authentication error.
	// An authentication error occurs if subscription key or authorization token is invalid, expired,
	// or does not match the region being used.
    #[error("subscription key or authorization token is invalid, expired, or does not match the region being used")]
    AuthenticationFailure,

    // BadRequest indicates that one or more recognition parameters are invalid or the audio format is not supported.
    #[error("one or more recognition parameters are invalid or the audio format is not supported")]
    BadRequest,

    // TooManyRequests indicates that the number of parallel requests exceeded the number of allowed concurrent transcriptions for the subscription.
    #[error("the number of parallel requests exceeded the number of allowed concurrent transcriptions for the subscription")]
    TooManyRequests,

    // Forbidden indicates that the free subscription used by the request ran out of quota.
    #[error("the free subscription used by the request ran out of quota")]
    Forbidden,

    // ConnectionFailure indicates a connection error.
    #[error("connection error in cognitive services speech sdk")]
    ConnectionFailure,

    // ServiceTimeout indicates a time-out error when waiting for response from service.
    #[error("time-out error when waiting for response from speech service")]
    ServiceTimeout,

    // ServiceError indicates that an error is returned by the service.
    #[error("error is returned by speech service")]
    ServiceError,

    // ServiceUnavailable indicates that the service is currently unavailable.
    #[error("speech service currently unavailable")]
    ServiceUnavailable,

    // RuntimeError indicates an unexpected runtime error.
    #[error("unexpected runtime error in cognitive services speech sdk")]
    RuntimeError
}

type CancellationErrorDetails = serde_json::Value;

// CancellationReason defines the possible reasons a recognition result might be canceled.
enum CancellationReason {

    // Error indicates that an error occurred during speech recognition.
    Error(CancellationError, CancellationErrorDetails),

    // EndOfStream indicates that the end of the audio stream was reached.
    EndOfStream,

    // CancelledByUser indicates that request was cancelled by the user.
    CancelledByUser
}

enum OutputFormat {
    Simple,
    Detailed
}

enum ProfanityOption{
    Masked,
    Removed,
    Raw
}

struct PropertyCollection {
    // SpeechServiceConnectionKey is the Cognitive Services Speech Service subscription key. If you are using an
	// intent recognizer, you need to specify the LUIS endpoint key for your particular LUIS app. Under normal
	// circumstances, you shouldn't have to use this property directly.
    speech_service_connection_key: String,

    // SpeechServiceConnectionEndpoint is the Cognitive Services Speech Service endpoint (url).
	// Under normal circumstances, you shouldn't have to use this property directly.
	// NOTE: This endpoint is not the same as the endpoint used to obtain an access token.
    speech_service_connection_endpoint: SocketAddr,

    // SpeechServiceConnectionRegion is the Cognitive Services Speech Service region. Under normal circumstances,
	// you shouldn't have to use this property directly.
    speech_service_connection_region: String,

    // SpeechServiceAuthorizationToken is the Cognitive Services Speech Service authorization token (aka access token).
	// Under normal circumstances, you shouldn't have to use this property directly.
    speech_service_authorization_token: String,

    // SpeechServiceAuthorizationType is the Cognitive Services Speech Service authorization type. Currently unused.
    speech_service_authorization_type: Option<String>,

    // SpeechServiceConnectionEndpointID is the Cognitive Services Custom Speech Service endpoint id. Under normal
	// circumstances, you shouldn't have to use this property directly.
    // NOTE: The endpoint id is available in the Custom Speech Portal, listed under Endpoint Details.
    speech_service_connection_endpoint_id: String,

    // SpeechServiceConnectionHost is the Cognitive Services Speech Service host (url). Under normal circumstances,
	// you shouldn't have to use this property directly.
    service_speech_connection_host: String,

    // SpeechServiceConnectionProxyHostName is the host name of the proxy server used to connect to the Cognitive Services
	// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    speech_service_connection_proxy_host_name: String,

    // SpeechServiceConnectionProxyPort is the port of the proxy server used to connect to the Cognitive Services Speech
	// Service. Under normal circumstances, you shouldn't have to use this property directly.
    speech_service_connection_proxy_port: u16,

    // SpeechServiceConnectionProxyUserName is the user name of the proxy server used to connect to the Cognitive Services
	// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    speech_service_connection_proxy_user_name: String,

    // SpeechServiceConnectionProxyPassword is the password of the proxy server used to connect to the Cognitive Services
	// Speech Service. Under normal circumstances, you shouldn't have to use this property directly.
    speech_service_connection_proxy_password: String,

    // SpeechServiceConnectionTranslationToLanguages is the list languages used as target translation
	// languages. Under normal circumstances, you shouldn't have to use this property directly.

    speech_service_connection_translation_to_languages: Vec<String>,

    // SpeechServiceConnectionTranslationVoice is the name of the Cognitive Service Text to Speech Service voice. Under normal
	// circumstances, you shouldn't have to use this property directly.
    // NOTE: Valid voice names can be found at https://aka.ms/csspeech/voicenames.
    speech_service_connection_translation_voice: String,

    // SpeechServiceConnectionTranslationFeatures is the translation features. For internal use.
    speech_service_connection_translation_features: Vec<String>,

    // SpeechServiceConnectionIntentRegion is the Language Understanding Service region. Under normal circumstances, you
	// shouldn't have to use this property directly.
    speech_service_connection_intent_region: String,

    // This property is intended to be read-only. The SDK is using it internally.
    speech_service_connection_reco_mode: RecognitionMode,

    // SpeechServiceConnectionRecoLanguage is the spoken language to be recognized (in BCP-47 format). Under normal
	// circumstances, you shouldn't have to use this property directly.
    speech_service_connection_reco_language: String,

    // SpeechSessionID is the session id. This id is a universally unique identifier (aka UUID) representing a specific
	// binding of an audio input stream and the underlying speech recognition instance to which it is bound. Under normal
	// circumstances, you shouldn't have to use this property directly.
    speech_session_id: String,

    // SpeechServiceConnectionUserDefinedQueryParameters are the query parameters provided by users. They will be passed
	// to the service as URL query parameters.
    speech_service_connection_user_defined_query_parameters: std::collections::HashMap<String, String>,

    // SpeechServiceConnectionSynthLanguage is the spoken language to be synthesized (e.g. en-US)
    speech_service_connection_synth_language: String,

    // SpeechServiceConnectionSynthVoice is the name of the TTS voice to be used for speech synthesis
    speech_connection_synth_voice: String,

    // SpeechServiceConnectionSynthOutputFormat is the string to specify TTS output audio format.
    speech_connection_synth_output_format: String,

    // SpeechServiceConnectionSynthEnableCompressedAudioTransmission indicates if use compressed audio format
	// for speech synthesis audio transmission.
	// This property only affects when SpeechServiceConnectionSynthOutputFormat is set to a pcm format.
    speech_service_connection_synth_enable_compressed_audio_transmission: String,

    // SpeechServiceConnectionInitialSilenceTimeoutMs is the initial silence timeout value (in milliseconds) used by the service.
    speech_service_connection_initial_silence_timeout_ms: u32,

    // SpeechServiceConnectionEndSilenceTimeoutMs is the end silence timeout value (in milliseconds) used by the service.
    speech_service_connection_end_silence_timeout_ms: u32,

    // value specifying whether audio logging is enabled in the service or not
    speech_service_connection_enable_audio_logging: bool,

    // SpeechServiceConnectionAutoDetectSourceLanguages is the auto detect source languages
    speech_service_connection_auto_detect_source_lan: Vec<String>,

    // SpeechServiceConnectionAutoDetectSourceLanguageResult is the auto detect source language result,
    speech_service_connection_auto_detect_source_language_result: String,

    // SpeechServiceResponseProfanityOption is the requested Cognitive Services Speech Service response output profanity setting
    speech_service_response_profanity_op: ProfanityOption,

    // SpeechServiceResponsePostProcessingOption a string value specifying which post processing option should be used
	// by the service.
    speech_service_response_post_processing_option: PostProcessingOption,

    // SpeechServiceResponseRequestWordLevelTimestamps is a boolean value specifying whether to include word-level
	// timestamps in the response result.
    speech_service_response_request_word_level_time: bool,

    // SpeechServiceResponseStablePartialResultThreshold is the number of times a word has to be in partial results
	// to be returned
    speech_service_response_stable_partial_result_threshold: u8,

    // SpeechServiceResponseOutputFormatOption is a string value specifying the output format option in the response
	// result. Internal use only.
    speech_service_response_output_format_option: OutputFormat,

    // SpeechServiceResponseTranslationRequestStablePartialResult is a boolean value to request for stabilizing translation
	// partial results by omitting words in the end.
    speech_service_response_translation_request_stable_partial_result: bool,

    // SpeechServiceResponseRequestWordBoundary is a boolean value specifying whether to request WordBoundary events.
    speech_service_response_request_word_bou: bool,

    // SpeechServiceResponseRequestPunctuationBoundary is a boolean value specifying whether to request punctuation boundary
	// in WordBoundary Events. Default is true.
    speech_service_response_request_punctuation_boundary: bool,

    // SpeechServiceResponseRequestSentenceBoundary ia a boolean value specifying whether to request sentence boundary
	// in WordBoundary Events. Default is false.
    speech_service_request_response_sentence_boundary: bool,

    // SpeechServiceResponseJSONResult is the Cognitive Services Speech Service response output (in JSON format). This
	// property is available on recognition result objects only.
    speech_service_response_jsonresult: serde_json::Value,

    // SpeechServiceResponseRecognitionLatencyMs is the recognition latency in milliseconds. Read-only, available on final
	// speech/translation/intent results. This measures the latency between when an audio input is received by the SDK, and
	// the moment the final result is received from the service. The SDK computes the time difference between the last audio
	// fragment from the audio input that is contributing to the final result, and the time the final result is received from
	// the speech service.
    speech_service_response_recognition_latency_ms: u64,

    // SpeechServiceResponseSynthesisFirstByteLatencyMs is the speech synthesis first byte latency in milliseconds.
	// Read-only, available on final speech synthesis results.
	// This measures the latency between when the synthesis is started to be processed, and the moment the first byte audio is available.
    speech_service_response_synthesis_first_byte_latency_ms: u32,

    // SpeechServiceResponseSynthesisFinishLatencyMs is the speech synthesis all bytes latency in milliseconds.
	// Read-only, available on final speech synthesis results.
	// This measures the latency between when the synthesis is started to be processed, and the moment the whole audio is synthesized.
    speech_service_response_synthesis_finish_latency_ms: u64,

    // SpeechServiceResponseSynthesisUnderrunTimeMs is the underrun time for speech synthesis in milliseconds.
	// Read-only, available on results in SynthesisCompleted events.
	// This measures the total underrun time from AudioConfigPlaybackBufferLengthInMs is filled to synthesis completed.
    speech_service_response_synthesis_underrun_time_ms: u64,

    // SpeechServiceResponseSynthesisBackend indicates which backend the synthesis is finished by.
	// Read-only, available on speech synthesis results, except for the result in SynthesisStarted event
    speech_service_response_synthesis_backend: String,

    // CancellationDetailsReason is the cancellation reason. Currently unused.
    cancellation_details_reason: CancellationReason,

    // CancellationDetailsReasonText the cancellation text. Currently unused.
    cancellation_details_reason_text: String,

    // CancellationDetailsReasonDetailedText is the cancellation detailed text. Currently unused.
    cancellation_details_reason_detailed_text: String,

    // LanguageUnderstandingServiceResponseJSONResult is the Language Understanding Service response output (in JSON format).
	// Available via IntentRecognitionResult.Properties.
    language_understanding_service_response_json_result: serde_json::Value,

    // AudioConfigDeviceNameForCapture is the device name for audio capture. Under normal circumstances, you shouldn't have
	// to use this property directly.
    audio_config_device_name_for_capture: String,
    
    // AudioConfigNumberOfChannelsForCapture is the number of channels for audio capture. Internal use only.
    audio_config_number_of_channels_for_capture: u8,
    
    // AudioConfigSampleRateForCapture is the sample rate (in Hz) for audio capture. Internal use only.
    audio_config_sample_rate_for_capture: u32,

    // AudioConfigBitsPerSampleForCapture is the number of bits of each sample for audio capture. Internal use only.
    audio_config_bits_per_sample_for_capture: u8,

    // AudioConfigAudioSource is the audio source
    audio_config_audio_source: AudioSource,

    // AudioConfigDeviceNameForRender indicates the device name for audio render. Under normal circumstances,
	// you shouldn't have to use this property directly. Instead, use NewAudioConfigFromDefaultSpeakerOutput.
    audio_config_device_name_for_render: String,

    // AudioConfigPlaybackBufferLengthInMs indicates the playback buffer length in milliseconds, default is 50 milliseconds.
    audio_config_playback_buffer_length_in_ms: u32,

    // AudioProcessingOptions provides advanced configuration for audio input for features like Voice Activity Detection
	// and is provided in the form of a JSON string.
    audio_processing_options: serde_json::Value,

    // SpeechLogFilename is the file name to write logs.
    speech_log_file_name: String,

    // SegmentationSilenceTimeoutMs specifies a duration of detected silence, measured in milliseconds, after which
	// speech-to-text will determine a spoken phrase has ended and generate a final Recognized result. Configuring
	// this timeout may be helpful in situations where spoken input is significantly faster or slower than usual and
	// default segmentation behavior consistently yields results that are too long or too short. Segmentation timeout
	// values that are inappropriately high or low can negatively affect speech-to-text accuracy; this property should
	// be carefully configured and the resulting behavior should be thoroughly validated as intended.
	//
	// For more information about timeout configuration that includes discussion of default behaviors, please visit
	// https://aka.ms/csspeech/timeouts.
    segmentation_silence_timeout_ms: u32,

    // ConversationApplicationID is the identifier used to connect to the backend service.
    conversation_application_id: String,

    // ConversationDialogType is the type of dialog backend to connect to.
    conversation_dialog_type: String,

    // ConversationInitialSilenceTimeout is the silence timeout for listening.
    conversation_initial_silence_timeout: u32,

    // ConversationFromID is the FromId to be used on speech recognition activities.
    conversation_from_id: String,

    // ConversationConversationID is the ConversationId for the session.
    conversation_conversation_id: String,

    // ConversationCustomVoiceDeploymentIDs is a list of custom voice deployment ids.
    conversation_custom_voice_deployment_ids: Vec<String>,

    // DataBufferTimeStamp is the time stamp associated to data buffer written by client when using Pull/Push
	// audio input streams.
	// The time stamp is a 64-bit value with a resolution of 90 kHz. It is the same as the presentation timestamp
	// in an MPEG transport stream. See https://en.wikipedia.org/wiki/Presentation_timestamp
    data_buffer_time_stamp: u64,

    // DataBufferUserID is the user id associated to data buffer written by client when using Pull/Push audio
	// input streams.
    data_buffer_user_id: String
}

impl PropertyCollection {
    fn speech_service_connection_url() -> String {
        todo!()
    }

    // SpeechServiceResponseRequestDetailedResultTrueFalse the requested Cognitive Services Speech Service response output
	// format (simple or detailed). Under normal circumstances, you shouldn't have to use this property directly.
    fn speech_service_response_request_detailed_result_true_false() -> bool {
        todo!()
    }

    // SpeechServiceResponseRequestProfanityFilterTrueFalse is the requested Cognitive Services Speech Service response
	// output profanity level. Currently unused.
    fn speech_service_response_request_profanity_filter_true_false() -> bool {
        todo!()
    }
}

enum RecognitionMode {
    Interactive,
    Conversation,
    Dictation
}

enum PostProcessingOption {
    TrueText
}

enum AudioSource {
    Microphones,
    File,
    Stream
}

enum ResultReason {
    // NoMatch indicates speech could not be recognized. More details can be found in the NoMatchDetails object.
    NoMatch,

    // Canceled indicates that the recognition was canceled. More details can be found using the CancellationDetails object.
    Canceled,

    // RecognizingSpeech indicates the speech result contains hypothesis text.
    RecognizingSpeech,

    // RecognizedSpeech indicates the speech result contains final text that has been recognized.
	// Speech Recognition is now complete for this phrase.
    RecognizedSpeech,

    // RecognizingIntent indicates the intent result contains hypothesis text and intent.
    RecognizingIntent,

    // RecognizedIntent indicates the intent result contains final text and intent.
	// Speech Recognition and Intent determination are now complete for this phrase.
    RecognizedIntent,

    // TranslatingSpeech indicates the translation result contains hypothesis text and its translation(s).
    TranslatingSpeech,

    // TranslatedSpeech indicates the translation result contains final text and corresponding translation(s).
	// Speech Recognition and Translation are now complete for this phrase.
    TranslatedSpeech,

    // SynthesizingAudio indicates the synthesized audio result contains a non-zero amount of audio data
    SynthesizingAudio,

    // SynthesizingAudioCompleted indicates the synthesized audio is now complete for this phrase.
    SynthesisCompleted,

    // RecognizingKeyword indicates the speech result contains (unverified) keyword text.
    RecognizingKeyword,

    // RecognizedKeyword indicates that keyword recognition completed recognizing the given keyword.
    RecognizedKeyword,

    // SynthesizingAudioStarted indicates the speech synthesis is now started
    SynthesizingAudioStarted,

    // EnrollingVoiceProfile indicates the voice profile is being enrolling and customers need to send more audio to create a voice profile.
    EnrollingVoiceProfile,

    // EnrolledVoiceProfile indicates the voice profile has been enrolled.
    EnrolledVoiceProfile,

    // RecognizedSpeakers indicates some speakers have been successfully identified.
    RecognizedSpeakers,

    // RecognizedSpeaker indicates one speaker has been successfully verified.
    RecognizedSpeaker,

    // ResetVoiceProfile indicates the voice profile has been reset successfully.
    ResetVoiceProfile,

    // DeletedVoiceProfile indicates the voice profile has been deleted successfully.
    DeletedVoiceProfile,

    // VoicesListRetrieved indicates the voices list has been retrieved successfully.
    VoicesListRetrieved
}

type SPXHandle = usize;

enum ServicePropertyChannel {
    UriQueryParameter
}

enum SpeechSynthesisBoundaryType {
    WordBoundary,
    PunctuationBoundary,
    SentenceBoundary
}

enum SpeechSynthesisOutputFormat {
    	// Raw8Khz8BitMonoMULaw stands for raw-8khz-8bit-mono-mulaw
	Raw8Khz8BitMonoMULaw,

	// Riff16Khz16KbpsMonoSiren stands for riff-16khz-16kbps-mono-siren
	// Unsupported by the service. Do not use this value.
	Riff16Khz16KbpsMonoSiren,

	// Audio16Khz16KbpsMonoSiren stands for audio-16khz-16kbps-mono-siren
	// Unsupported by the service. Do not use this value.
	Audio16Khz16KbpsMonoSiren,

	// Audio16Khz32KBitRateMonoMp3 stands for audio-16khz-32kbitrate-mono-mp3
	Audio16Khz32KBitRateMonoMp3,

	// Audio16Khz128KBitRateMonoMp3 stands for audio-16khz-128kbitrate-mono-mp3
	Audio16Khz128KBitRateMonoMp3,

	// Audio16Khz64KBitRateMonoMp3 stands for audio-16khz-64kbitrate-mono-mp3
	Audio16Khz64KBitRateMonoMp3,

	// Audio24Khz48KBitRateMonoMp3 stands for audio-24khz-48kbitrate-mono-mp3
	Audio24Khz48KBitRateMonoMp3,

	// Audio24Khz96KBitRateMonoMp3 stands for audio-24khz-96kbitrate-mono-mp3
	Audio24Khz96KBitRateMonoMp3,

	// Audio24Khz160KBitRateMonoMp3 stands for audio-24khz-160kbitrate-mono-mp3
	Audio24Khz160KBitRateMonoMp3,

	// Raw16Khz16BitMonoTrueSilk stands for raw-16khz-16bit-mono-truesilk
	Raw16Khz16BitMonoTrueSilk,

	// Riff16Khz16BitMonoPcm stands for riff-16khz-16bit-mono-pcm
	Riff16Khz16BitMonoPcm,

	// Riff8Khz16BitMonoPcm stands for riff-8khz-16bit-mono-pcm
	Riff8Khz16BitMonoPc,

	// Riff24Khz16BitMonoPcm stands for riff-24khz-16bit-mono-pcm
	Riff24Khz16BitMonoPcm,

	// Riff8Khz8BitMonoMULaw stands for riff-8khz-8bit-mono-mulaw
	Riff8Khz8BitMonoMULaw,

	// Raw16Khz16BitMonoPcm stands for raw-16khz-16bit-mono-pcm
	Raw16Khz16BitMonoPcm,

	// Raw24Khz16BitMonoPcm stands for raw-24khz-16bit-mono-pcm
	Raw24Khz16BitMonoPcm,

	// Raw8Khz16BitMonoPcm stands for raw-8khz-16bit-mono-pcm
	Raw8Khz16BitMonoPcm,

	// Ogg16Khz16BitMonoOpus stands for ogg-16khz-16bit-mono-opus
	Ogg16Khz16BitMonoOpus,

	// Ogg24Khz16BitMonoOpus stands for ogg-24khz-16bit-mono-opus
	Ogg24Khz16BitMonoOpus,

	// Raw48Khz16BitMonoPcm stands for raw-48khz-16bit-mono-pcm
	Raw48Khz16BitMonoPcm,

	// Riff48Khz16BitMonoPcm stands for riff-48khz-16bit-mono-pcm
	Riff48Khz16BitMonoPcm,

	// Audio48Khz96KBitRateMonoMp3 stands for audio-48khz-96kbitrate-mono-mp3
	Audio48Khz96KBitRateMonoMp3,

	// Audio48Khz192KBitRateMonoMp3 stands for audio-48khz-192kbitrate-mono-mp3
	Audio48Khz192KBitRateMonoMp3,

	// Ogg48Khz16BitMonoOpus stands for ogg-48khz-16bit-mono-opus
	Ogg48Khz16BitMonoOpus,

	// Webm16Khz16BitMonoOpus stands for webm-16khz-16bit-mono-opus
	Webm16Khz16BitMonoOpus,

	// Webm24Khz16BitMonoOpus stands for webm-24khz-16bit-mono-opus
	Webm24Khz16BitMonoOpus,

	// Raw24Khz16BitMonoTrueSilk stands for raw-24khz-16bit-mono-truesilk
	Raw24Khz16BitMonoTrueSilk,

	// Raw8Khz8BitMonoALaw stands for raw-8khz-8bit-mono-alaw
	Raw8Khz8BitMonoALaw,

	// Riff8Khz8BitMonoALaw stands for riff-8khz-8bit-mono-alaw
	Riff8Khz8BitMonoALaw,

	// Webm24Khz16Bit24KbpsMonoOpus stands for webm-24khz-16bit-24kbps-mono-opus
	// Audio compressed by OPUS codec in a WebM container, with bitrate of 24kbps, optimized for IoT scenario.
	Webm24Khz16Bit24KbpsMonoOpus,

	// Audio16Khz16Bit32KbpsMonoOpus stands for audio-16khz-16bit-32kbps-mono-opus
	// Audio compressed by OPUS codec without container, with bitrate of 32kbps.
	Audio16Khz16Bit32KbpsMonoOpus,

	// Audio24Khz16Bit48KbpsMonoOpus stands for audio-24khz-16bit-48kbps-mono-opus
	// Audio compressed by OPUS codec without container, with bitrate of 48kbps.
	Audio24Khz16Bit48KbpsMonoOpus,

	// Audio24Khz16Bit24KbpsMonoOpus stands for audio-24khz-16bit-24kbps-mono-opus
	// Audio compressed by OPUS codec without container, with bitrate of 24kbps.
	Audio24Khz16Bit24KbpsMonoOpus,
}

enum StreamStatus {
    // StreamStatusUnknown indicates the audio data stream status is unknown.
	StreamStatusUnknown,

	// StreamStatusNoData indicates that the audio data stream contains no data.
	StreamStatusNoData,

	// StreamStatusPartialData indicates the audio data stream contains partial data of a speak request.
	StreamStatusPartialData,

	// StreamStatusAllData indicates the audio data stream contains all data of a speak request.
	StreamStatusAllData,

	// StreamStatusCanceled indicates the audio data stream was canceled.
	StreamStatusCanceled,
}

enum SynthesisVoiceGender {
    GenderUnknown,
    Female,
    Male
}

enum SynthesisVoiceType {
    	// OnlineNeural indicates online neural voice.
	OnlineNeural,

	// OnlineStandard indicates online standard voice.
	OnlineStandard,

	// OfflineNeural indicates offline neural voice.
	OfflineNeural,

	// OfflineStandard indicates offline started voice.
	OfflineStandard,
}

enum VoiceProfileType {
    // Text independent speaker identification
	TextIndependentIdentification,

	// Text dependent speaker verification
	TextDependentVerification,

	// Text independent speaker verification
	TextIndependentVerification,
}