use windows::{
    Win32::Media::MediaFoundation::{
        FORMAT_VideoInfo, FORMAT_VideoInfo2, MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION,
        MF_MT_AAC_PAYLOAD_TYPE, MF_MT_ALL_SAMPLES_INDEPENDENT, MF_MT_AM_FORMAT_TYPE,
        MF_MT_ARBITRARY_FORMAT, MF_MT_ARBITRARY_HEADER, MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
        MF_MT_AUDIO_BITS_PER_SAMPLE, MF_MT_AUDIO_BLOCK_ALIGNMENT, MF_MT_AUDIO_CHANNEL_MASK,
        MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND, MF_MT_AUDIO_FOLDDOWN_MATRIX,
        MF_MT_AUDIO_NUM_CHANNELS, MF_MT_AUDIO_PREFER_WAVEFORMATEX, MF_MT_AUDIO_SAMPLES_PER_BLOCK,
        MF_MT_AUDIO_SAMPLES_PER_SECOND, MF_MT_AUDIO_VALID_BITS_PER_SAMPLE,
        MF_MT_AUDIO_WMADRC_AVGREF, MF_MT_AUDIO_WMADRC_AVGTARGET, MF_MT_AUDIO_WMADRC_PEAKREF,
        MF_MT_AUDIO_WMADRC_PEAKTARGET, MF_MT_AVG_BIT_ERROR_RATE, MF_MT_AVG_BITRATE,
        MF_MT_COMPRESSED, MF_MT_CUSTOM_VIDEO_PRIMARIES, MF_MT_DEFAULT_STRIDE, MF_MT_DRM_FLAGS,
        MF_MT_DV_AAUX_CTRL_PACK_0, MF_MT_DV_AAUX_CTRL_PACK_1, MF_MT_DV_AAUX_SRC_PACK_0,
        MF_MT_DV_AAUX_SRC_PACK_1, MF_MT_DV_VAUX_CTRL_PACK, MF_MT_DV_VAUX_SRC_PACK,
        MF_MT_FIXED_SIZE_SAMPLES, MF_MT_FRAME_RATE, MF_MT_FRAME_RATE_RANGE_MAX,
        MF_MT_FRAME_RATE_RANGE_MIN, MF_MT_FRAME_SIZE, MF_MT_GEOMETRIC_APERTURE,
        MF_MT_IMAGE_LOSS_TOLERANT, MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE,
        MF_MT_MAX_KEYFRAME_SPACING, MF_MT_MINIMUM_DISPLAY_APERTURE, MF_MT_MPEG_SEQUENCE_HEADER,
        MF_MT_MPEG_START_TIME_CODE, MF_MT_MPEG2_FLAGS, MF_MT_MPEG2_LEVEL, MF_MT_MPEG2_PROFILE,
        MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY, MF_MT_MPEG4_SAMPLE_DESCRIPTION, MF_MT_ORIGINAL_4CC,
        MF_MT_ORIGINAL_WAVE_FORMAT_TAG, MF_MT_PAD_CONTROL_FLAGS, MF_MT_PALETTE,
        MF_MT_PAN_SCAN_APERTURE, MF_MT_PAN_SCAN_ENABLED, MF_MT_PIXEL_ASPECT_RATIO,
        MF_MT_SAMPLE_SIZE, MF_MT_SOURCE_CONTENT_HINT, MF_MT_SUBTYPE, MF_MT_TRANSFER_FUNCTION,
        MF_MT_USER_DATA, MF_MT_VIDEO_CHROMA_SITING, MF_MT_VIDEO_LIGHTING,
        MF_MT_VIDEO_NOMINAL_RANGE, MF_MT_VIDEO_PRIMARIES, MF_MT_WRAPPED_TYPE, MF_MT_YUV_MATRIX,
        MFAudioFormat_AAC, MFAudioFormat_ADTS, MFAudioFormat_DRM, MFAudioFormat_DTS,
        MFAudioFormat_Dolby_AC3_SPDIF, MFAudioFormat_Float, MFAudioFormat_MP3, MFAudioFormat_MPEG,
        MFAudioFormat_MSP1, MFAudioFormat_PCM, MFAudioFormat_WMASPDIF,
        MFAudioFormat_WMAudio_Lossless, MFAudioFormat_WMAudioV8, MFAudioFormat_WMAudioV9,
        MFMediaType_Audio, MFMediaType_Binary, MFMediaType_FileTransfer, MFMediaType_HTML,
        MFMediaType_Image, MFMediaType_Protected, MFMediaType_SAMI, MFMediaType_Script,
        MFMediaType_Video, MFVideoFormat_AI44, MFVideoFormat_ARGB32, MFVideoFormat_AYUV,
        MFVideoFormat_DV25, MFVideoFormat_DV50, MFVideoFormat_DVH1, MFVideoFormat_DVSD,
        MFVideoFormat_DVSL, MFVideoFormat_H264, MFVideoFormat_I420, MFVideoFormat_IYUV,
        MFVideoFormat_M4S2, MFVideoFormat_MJPG, MFVideoFormat_MP4S, MFVideoFormat_MP4V,
        MFVideoFormat_MP43, MFVideoFormat_MPG1, MFVideoFormat_MSS1, MFVideoFormat_MSS2,
        MFVideoFormat_NV11, MFVideoFormat_NV12, MFVideoFormat_P010, MFVideoFormat_P016,
        MFVideoFormat_P210, MFVideoFormat_P216, MFVideoFormat_RGB8, MFVideoFormat_RGB24,
        MFVideoFormat_RGB32, MFVideoFormat_RGB555, MFVideoFormat_RGB565, MFVideoFormat_UYVY,
        MFVideoFormat_WMV1, MFVideoFormat_WMV2, MFVideoFormat_WMV3, MFVideoFormat_WVC1,
        MFVideoFormat_Y41P, MFVideoFormat_Y41T, MFVideoFormat_Y210, MFVideoFormat_Y216,
        MFVideoFormat_Y410, MFVideoFormat_Y416, MFVideoFormat_YUY2, MFVideoFormat_YV12,
        MFVideoFormat_YVYU, MFVideoFormat_v210, MFVideoFormat_v410,
    },
    core::GUID,
};

macro_rules! match_guid_constants {
    ( $name:ident, $($guid_constant:ident),* $(,)* ) => {
        #[allow(unreachable_patterns)]
        #[allow(non_upper_case_globals)]
        match $name {
            $(
                $guid_constant => Some(stringify!($guid_constant)),
            )*
            _ => None
        }
    };
}

pub fn get_guid_name_const(guid: GUID) -> Option<&'static str> {
    match_guid_constants!(
        guid,
        MF_MT_MAJOR_TYPE,
        MF_MT_MAJOR_TYPE,
        MF_MT_SUBTYPE,
        MF_MT_ALL_SAMPLES_INDEPENDENT,
        MF_MT_FIXED_SIZE_SAMPLES,
        MF_MT_COMPRESSED,
        MF_MT_SAMPLE_SIZE,
        MF_MT_WRAPPED_TYPE,
        MF_MT_AUDIO_NUM_CHANNELS,
        MF_MT_AUDIO_SAMPLES_PER_SECOND,
        MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND,
        MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
        MF_MT_AUDIO_BLOCK_ALIGNMENT,
        MF_MT_AUDIO_BITS_PER_SAMPLE,
        MF_MT_AUDIO_VALID_BITS_PER_SAMPLE,
        MF_MT_AUDIO_SAMPLES_PER_BLOCK,
        MF_MT_AUDIO_CHANNEL_MASK,
        MF_MT_AUDIO_FOLDDOWN_MATRIX,
        MF_MT_AUDIO_WMADRC_PEAKREF,
        MF_MT_AUDIO_WMADRC_PEAKTARGET,
        MF_MT_AUDIO_WMADRC_AVGREF,
        MF_MT_AUDIO_WMADRC_AVGTARGET,
        MF_MT_AUDIO_PREFER_WAVEFORMATEX,
        MF_MT_AAC_PAYLOAD_TYPE,
        MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION,
        MF_MT_FRAME_SIZE,
        MF_MT_FRAME_RATE,
        MF_MT_FRAME_RATE_RANGE_MAX,
        MF_MT_FRAME_RATE_RANGE_MIN,
        MF_MT_PIXEL_ASPECT_RATIO,
        MF_MT_DRM_FLAGS,
        MF_MT_PAD_CONTROL_FLAGS,
        MF_MT_SOURCE_CONTENT_HINT,
        MF_MT_VIDEO_CHROMA_SITING,
        MF_MT_INTERLACE_MODE,
        MF_MT_TRANSFER_FUNCTION,
        MF_MT_VIDEO_PRIMARIES,
        MF_MT_CUSTOM_VIDEO_PRIMARIES,
        MF_MT_YUV_MATRIX,
        MF_MT_VIDEO_LIGHTING,
        MF_MT_VIDEO_NOMINAL_RANGE,
        MF_MT_GEOMETRIC_APERTURE,
        MF_MT_MINIMUM_DISPLAY_APERTURE,
        MF_MT_PAN_SCAN_APERTURE,
        MF_MT_PAN_SCAN_ENABLED,
        MF_MT_AVG_BITRATE,
        MF_MT_AVG_BIT_ERROR_RATE,
        MF_MT_MAX_KEYFRAME_SPACING,
        MF_MT_DEFAULT_STRIDE,
        MF_MT_PALETTE,
        MF_MT_USER_DATA,
        MF_MT_AM_FORMAT_TYPE,
        MF_MT_MPEG_START_TIME_CODE,
        MF_MT_MPEG2_PROFILE,
        MF_MT_MPEG2_LEVEL,
        MF_MT_MPEG2_FLAGS,
        MF_MT_MPEG_SEQUENCE_HEADER,
        MF_MT_DV_AAUX_SRC_PACK_0,
        MF_MT_DV_AAUX_CTRL_PACK_0,
        MF_MT_DV_AAUX_SRC_PACK_1,
        MF_MT_DV_AAUX_CTRL_PACK_1,
        MF_MT_DV_VAUX_SRC_PACK,
        MF_MT_DV_VAUX_CTRL_PACK,
        MF_MT_ARBITRARY_HEADER,
        MF_MT_ARBITRARY_FORMAT,
        MF_MT_IMAGE_LOSS_TOLERANT,
        MF_MT_MPEG4_SAMPLE_DESCRIPTION,
        MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY,
        MF_MT_ORIGINAL_4CC,
        MF_MT_ORIGINAL_WAVE_FORMAT_TAG,
        // Media types
        MFMediaType_Audio,
        MFMediaType_Video,
        MFMediaType_Protected,
        MFMediaType_SAMI,
        MFMediaType_Script,
        MFMediaType_Image,
        MFMediaType_HTML,
        MFMediaType_Binary,
        MFMediaType_FileTransfer,
        MFVideoFormat_AI44,   //     FCC('AI44')
        MFVideoFormat_ARGB32, //   D3DFMT_A8R8G8B8
        MFVideoFormat_AYUV,   //     FCC('AYUV')
        MFVideoFormat_DV25,   //     FCC('dv25')
        MFVideoFormat_DV50,   //     FCC('dv50')
        MFVideoFormat_DVH1,   //     FCC('dvh1')
        MFVideoFormat_DVSD,   //     FCC('dvsd')
        MFVideoFormat_DVSL,   //     FCC('dvsl')
        MFVideoFormat_H264,   //     FCC('H264')
        MFVideoFormat_I420,   //     FCC('I420')
        MFVideoFormat_IYUV,   //     FCC('IYUV')
        MFVideoFormat_M4S2,   //     FCC('M4S2')
        MFVideoFormat_MJPG,
        MFVideoFormat_MP43,   //     FCC('MP43')
        MFVideoFormat_MP4S,   //     FCC('MP4S')
        MFVideoFormat_MP4V,   //     FCC('MP4V')
        MFVideoFormat_MPG1,   //     FCC('MPG1')
        MFVideoFormat_MSS1,   //     FCC('MSS1')
        MFVideoFormat_MSS2,   //     FCC('MSS2')
        MFVideoFormat_NV11,   //     FCC('NV11')
        MFVideoFormat_NV12,   //     FCC('NV12')
        MFVideoFormat_P010,   //     FCC('P010')
        MFVideoFormat_P016,   //     FCC('P016')
        MFVideoFormat_P210,   //     FCC('P210')
        MFVideoFormat_P216,   //     FCC('P216')
        MFVideoFormat_RGB24,  //    D3DFMT_R8G8B8
        MFVideoFormat_RGB32,  //    D3DFMT_X8R8G8B8
        MFVideoFormat_RGB555, //   D3DFMT_X1R5G5B5
        MFVideoFormat_RGB565, //   D3DFMT_R5G6B5
        MFVideoFormat_RGB8,
        MFVideoFormat_UYVY, //     FCC('UYVY')
        MFVideoFormat_v210, //     FCC('v210')
        MFVideoFormat_v410, //     FCC('v410')
        MFVideoFormat_WMV1, //     FCC('WMV1')
        MFVideoFormat_WMV2, //     FCC('WMV2')
        MFVideoFormat_WMV3, //     FCC('WMV3')
        MFVideoFormat_WVC1, //     FCC('WVC1')
        MFVideoFormat_Y210, //     FCC('Y210')
        MFVideoFormat_Y216, //     FCC('Y216')
        MFVideoFormat_Y410, //     FCC('Y410')
        MFVideoFormat_Y416, //     FCC('Y416')
        MFVideoFormat_Y41P,
        MFVideoFormat_Y41T,
        MFVideoFormat_YUY2, //     FCC('YUY2')
        MFVideoFormat_YV12, //     FCC('YV12')
        MFVideoFormat_YVYU,
        MFAudioFormat_PCM,              //              WAVE_FORMAT_PCM
        MFAudioFormat_Float,            //            WAVE_FORMAT_IEEE_FLOAT
        MFAudioFormat_DTS,              //              WAVE_FORMAT_DTS
        MFAudioFormat_Dolby_AC3_SPDIF,  //  WAVE_FORMAT_DOLBY_AC3_SPDIF
        MFAudioFormat_DRM,              //              WAVE_FORMAT_DRM
        MFAudioFormat_WMAudioV8,        //        WAVE_FORMAT_WMAUDIO2
        MFAudioFormat_WMAudioV9,        //        WAVE_FORMAT_WMAUDIO3
        MFAudioFormat_WMAudio_Lossless, // WAVE_FORMAT_WMAUDIO_LOSSLESS
        MFAudioFormat_WMASPDIF,         //         WAVE_FORMAT_WMASPDIF
        MFAudioFormat_MSP1,             //             WAVE_FORMAT_WMAVOICE9
        MFAudioFormat_MP3,              //              WAVE_FORMAT_MPEGLAYER3
        MFAudioFormat_MPEG,             //             WAVE_FORMAT_MPEG
        MFAudioFormat_AAC,              //              WAVE_FORMAT_MPEG_HEAAC
        MFAudioFormat_ADTS,             //             WAVE_FORMAT_MPEG_ADTS_AAC
        FORMAT_VideoInfo,
        FORMAT_VideoInfo2,
    )
}
