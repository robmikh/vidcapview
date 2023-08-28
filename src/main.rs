use std::io::Write;

use windows::{
    core::{Array, ComInterface, Result, GUID},
    Win32::{
        Foundation::BOOL,
        Media::MediaFoundation::{
            FORMAT_VideoInfo, FORMAT_VideoInfo2, IMFActivate, IMFAttributes, IMFMediaSource,
            IMFMediaType, MFAudioFormat_AAC, MFAudioFormat_ADTS, MFAudioFormat_DRM,
            MFAudioFormat_DTS, MFAudioFormat_Dolby_AC3_SPDIF, MFAudioFormat_Float,
            MFAudioFormat_MP3, MFAudioFormat_MPEG, MFAudioFormat_MSP1, MFAudioFormat_PCM,
            MFAudioFormat_WMASPDIF, MFAudioFormat_WMAudioV8, MFAudioFormat_WMAudioV9,
            MFAudioFormat_WMAudio_Lossless, MFCreateAttributes, MFEnumDeviceSources,
            MFMediaType_Audio, MFMediaType_Binary, MFMediaType_FileTransfer, MFMediaType_HTML,
            MFMediaType_Image, MFMediaType_Protected, MFMediaType_SAMI, MFMediaType_Script,
            MFMediaType_Video, MFStartup, MFVideoFormat_AI44, MFVideoFormat_ARGB32,
            MFVideoFormat_AYUV, MFVideoFormat_DV25, MFVideoFormat_DV50, MFVideoFormat_DVH1,
            MFVideoFormat_DVSD, MFVideoFormat_DVSL, MFVideoFormat_H264, MFVideoFormat_I420,
            MFVideoFormat_IYUV, MFVideoFormat_M4S2, MFVideoFormat_MJPG, MFVideoFormat_MP43,
            MFVideoFormat_MP4S, MFVideoFormat_MP4V, MFVideoFormat_MPG1, MFVideoFormat_MSS1,
            MFVideoFormat_MSS2, MFVideoFormat_NV11, MFVideoFormat_NV12, MFVideoFormat_P010,
            MFVideoFormat_P016, MFVideoFormat_P210, MFVideoFormat_P216, MFVideoFormat_RGB24,
            MFVideoFormat_RGB32, MFVideoFormat_RGB555, MFVideoFormat_RGB565, MFVideoFormat_RGB8,
            MFVideoFormat_UYVY, MFVideoFormat_WMV1, MFVideoFormat_WMV2, MFVideoFormat_WMV3,
            MFVideoFormat_WVC1, MFVideoFormat_Y210, MFVideoFormat_Y216, MFVideoFormat_Y410,
            MFVideoFormat_Y416, MFVideoFormat_Y41P, MFVideoFormat_Y41T, MFVideoFormat_YUY2,
            MFVideoFormat_YV12, MFVideoFormat_YVYU, MFVideoFormat_v210, MFVideoFormat_v410,
            MFSTARTUP_FULL, MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
            MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
            MF_E_ATTRIBUTENOTFOUND, MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION,
            MF_MT_AAC_PAYLOAD_TYPE, MF_MT_ALL_SAMPLES_INDEPENDENT, MF_MT_AM_FORMAT_TYPE,
            MF_MT_ARBITRARY_FORMAT, MF_MT_ARBITRARY_HEADER, MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
            MF_MT_AUDIO_BITS_PER_SAMPLE, MF_MT_AUDIO_BLOCK_ALIGNMENT, MF_MT_AUDIO_CHANNEL_MASK,
            MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND, MF_MT_AUDIO_FOLDDOWN_MATRIX,
            MF_MT_AUDIO_NUM_CHANNELS, MF_MT_AUDIO_PREFER_WAVEFORMATEX,
            MF_MT_AUDIO_SAMPLES_PER_BLOCK, MF_MT_AUDIO_SAMPLES_PER_SECOND,
            MF_MT_AUDIO_VALID_BITS_PER_SAMPLE, MF_MT_AUDIO_WMADRC_AVGREF,
            MF_MT_AUDIO_WMADRC_AVGTARGET, MF_MT_AUDIO_WMADRC_PEAKREF,
            MF_MT_AUDIO_WMADRC_PEAKTARGET, MF_MT_AVG_BITRATE, MF_MT_AVG_BIT_ERROR_RATE,
            MF_MT_COMPRESSED, MF_MT_CUSTOM_VIDEO_PRIMARIES, MF_MT_DEFAULT_STRIDE, MF_MT_DRM_FLAGS,
            MF_MT_DV_AAUX_CTRL_PACK_0, MF_MT_DV_AAUX_CTRL_PACK_1, MF_MT_DV_AAUX_SRC_PACK_0,
            MF_MT_DV_AAUX_SRC_PACK_1, MF_MT_DV_VAUX_CTRL_PACK, MF_MT_DV_VAUX_SRC_PACK,
            MF_MT_FIXED_SIZE_SAMPLES, MF_MT_FRAME_RATE, MF_MT_FRAME_RATE_RANGE_MAX,
            MF_MT_FRAME_RATE_RANGE_MIN, MF_MT_FRAME_SIZE, MF_MT_GEOMETRIC_APERTURE,
            MF_MT_IMAGE_LOSS_TOLERANT, MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE,
            MF_MT_MAX_KEYFRAME_SPACING, MF_MT_MINIMUM_DISPLAY_APERTURE, MF_MT_MPEG2_FLAGS,
            MF_MT_MPEG2_LEVEL, MF_MT_MPEG2_PROFILE, MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY,
            MF_MT_MPEG4_SAMPLE_DESCRIPTION, MF_MT_MPEG_SEQUENCE_HEADER, MF_MT_MPEG_START_TIME_CODE,
            MF_MT_ORIGINAL_4CC, MF_MT_ORIGINAL_WAVE_FORMAT_TAG, MF_MT_PAD_CONTROL_FLAGS,
            MF_MT_PALETTE, MF_MT_PAN_SCAN_APERTURE, MF_MT_PAN_SCAN_ENABLED,
            MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SAMPLE_SIZE, MF_MT_SOURCE_CONTENT_HINT, MF_MT_SUBTYPE,
            MF_MT_TRANSFER_FUNCTION, MF_MT_USER_DATA, MF_MT_VIDEO_CHROMA_SITING,
            MF_MT_VIDEO_LIGHTING, MF_MT_VIDEO_NOMINAL_RANGE, MF_MT_VIDEO_PRIMARIES,
            MF_MT_WRAPPED_TYPE, MF_MT_YUV_MATRIX, MF_VERSION,
        },
        System::{
            Com::StructuredStorage::{PropVariantClear, PROPVARIANT},
            Variant::{VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR},
            WinRT::{RoInitialize, RO_INIT_MULTITHREADED},
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        RoInitialize(RO_INIT_MULTITHREADED)?;
    }
    unsafe { MFStartup(MF_VERSION, MFSTARTUP_FULL)? }

    let attributes = unsafe {
        let mut attributes = None;
        MFCreateAttributes(&mut attributes, 1)?;
        attributes.unwrap()
    };

    unsafe {
        attributes.SetGUID(
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
        )?;
    }

    let sources = unsafe {
        let mut data = std::ptr::null_mut();
        let mut len = 0;
        MFEnumDeviceSources(&attributes, &mut data, &mut len)?;
        Array::<IMFActivate>::from_raw_parts(data as _, len)
    };
    let sources: Vec<IMFActivate> = sources
        .as_slice()
        .iter()
        .map(|source| source.as_ref().unwrap().clone())
        .collect();

    if sources.is_empty() {
        println!("No video capture devices found!");
    } else {
        if let Some(source) = select_source(&sources)? {
            let display_name = get_friendly_name(&source)?.unwrap_or("Unknown".to_owned());
            println!("Using {}...", display_name);

            // Activate the media source
            let media_source: IMFMediaSource = unsafe { source.ActivateObject()? };

            // Enumerate supported formats
            enum_formats(&media_source)?;
        } else {
            // Do nothing
        }
    }

    Ok(())
}

// https://learn.microsoft.com/en-us/windows/win32/medfound/how-to-set-the-video-capture-format
fn enum_formats(media_source: &IMFMediaSource) -> Result<()> {
    let presentation_descriptor = unsafe { media_source.CreatePresentationDescriptor()? };

    // TODO: Check each stream descriptor?
    let stream_descriptor = unsafe {
        let mut selected: BOOL = false.into();
        let mut stream_descriptor = None;
        presentation_descriptor.GetStreamDescriptorByIndex(
            0,
            &mut selected,
            &mut stream_descriptor,
        )?;
        stream_descriptor.unwrap()
    };

    let handler = unsafe { stream_descriptor.GetMediaTypeHandler()? };

    let num_types = unsafe { handler.GetMediaTypeCount()? };

    for i in 0..num_types {
        let media_type = unsafe { handler.GetMediaTypeByIndex(i)? };

        log_media_type(&media_type)?;
        println!("");
    }

    Ok(())
}

fn log_media_type(media_type: &IMFMediaType) -> Result<()> {
    let count = unsafe { media_type.GetCount()? };

    if count == 0 {
        println!("Empty media type");
    } else {
        let attributes: IMFAttributes = media_type.cast()?;

        for i in 0..count {
            log_attribute_value_by_index(&attributes, i)?;
        }
    }

    Ok(())
}

pub struct PropVariant(pub PROPVARIANT);

impl Drop for PropVariant {
    fn drop(&mut self) {
        let _ = unsafe { PropVariantClear(&mut self.0) };
    }
}

fn log_attribute_value_by_index(attributes: &IMFAttributes, index: u32) -> Result<()> {
    let (guid, variant) = unsafe {
        let mut guid = GUID::default();
        let mut variant = PROPVARIANT::default();
        attributes.GetItemByIndex(index, &mut guid, Some(&mut variant))?;
        (guid, PropVariant(variant))
    };

    let guid_name = get_guid_name(guid);

    let value_string = if let Some(value_string) = special_case_attribute_value(guid, &variant.0) {
        value_string
    } else {
        let vt_enum = unsafe { variant.0.Anonymous.Anonymous.vt };
        match vt_enum {
            VT_UI4 => format!("{}", unsafe {
                variant.0.Anonymous.Anonymous.Anonymous.ulVal
            }),
            VT_UI8 => format!("{}", unsafe {
                variant.0.Anonymous.Anonymous.Anonymous.uhVal
            }),
            VT_R8 => format!("{}", unsafe {
                variant.0.Anonymous.Anonymous.Anonymous.dblVal
            }),
            VT_CLSID => {
                let value_guid = unsafe { *variant.0.Anonymous.Anonymous.Anonymous.puuid };
                format!("{}", get_guid_name(value_guid))
            }
            VT_LPWSTR => {
                unsafe { variant.0.Anonymous.Anonymous.Anonymous.pwszVal.to_string() }.unwrap()
            }
            VT_VECTOR | VT_UI1 => "<<byte array>>".to_owned(),
            VT_UNKNOWN => "IUnknown".to_owned(),
            _ => format!("Unexpected attribute type (vt = {})", vt_enum.0),
        }
    };

    println!("{} - {}", guid_name, value_string);

    Ok(())
}

fn special_case_attribute_value(guid: GUID, var: &PROPVARIANT) -> Option<String> {
    match guid {
        MF_MT_FRAME_RATE
        | MF_MT_FRAME_RATE_RANGE_MAX
        | MF_MT_FRAME_RATE_RANGE_MIN
        | MF_MT_FRAME_SIZE
        | MF_MT_PIXEL_ASPECT_RATIO => {
            // Attributes that contain two packed 32-bit values
            let mut high = 0;
            let mut low = 0;
            unpack_2_u32_as_u64(
                unsafe { var.Anonymous.Anonymous.Anonymous.uhVal },
                &mut high,
                &mut low,
            );
            Some(format!("{} x {}", high, low))
        }
        MF_MT_GEOMETRIC_APERTURE | MF_MT_MINIMUM_DISPLAY_APERTURE | MF_MT_PAN_SCAN_APERTURE => {
            // TODO
            Some("TODO".to_owned())
        }
        _ => None,
    }
}

fn unpack_2_u32_as_u64(unpacked: u64, high: &mut u32, low: &mut u32) {
    *high = (unpacked >> 32) as u32;
    *low = unpacked as u32;
}

macro_rules! match_guid_constants {
    ( $name:ident, $($guid_constant:ident),* $(,)* ) => {
        match $name {
            $(
                $guid_constant => Some(stringify!($guid_constant)),
            )*
            _ => None
        }
    };
}

fn get_guid_name(guid: GUID) -> String {
    if let Some(guid_name) = get_guid_name_const(guid) {
        guid_name.to_owned()
    } else {
        format!("{:?}", guid)
    }
}

fn get_guid_name_const(guid: GUID) -> Option<&'static str> {
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

fn select_source(sources: &[IMFActivate]) -> Result<Option<IMFActivate>> {
    for (i, source) in sources.iter().enumerate() {
        let display_name = get_friendly_name(source)?.unwrap_or("Unknown".to_owned());
        println!("{:>3} - {}", i, display_name);
    }
    let index: usize;
    loop {
        print!("Please make a selection (q to quit): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().contains("q") {
            return Ok(None);
        }
        let input = input.trim();
        let selection: Option<usize> = match input.parse::<usize>() {
            Ok(selection) => {
                if selection < sources.len() {
                    Some(selection)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(selection) = selection {
            index = selection;
            break;
        } else {
            println!("Invalid input, '{}'!", input);
            continue;
        };
    }
    Ok(Some(sources[index].clone()))
}

fn get_friendly_name<T: ComInterface>(activate: &T) -> Result<Option<String>> {
    get_string_attribute(&activate.cast()?, &MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME)
}

fn get_string_attribute(
    attributes: &IMFAttributes,
    attribute_guid: &GUID,
) -> Result<Option<String>> {
    unsafe {
        match attributes.GetStringLength(attribute_guid) {
            Ok(mut length) => {
                let mut result = vec![0u16; (length + 1) as usize];
                attributes.GetString(attribute_guid, &mut result, Some(&mut length))?;
                result.resize(length as usize, 0);
                Ok(Some(String::from_utf16(&result).unwrap()))
            }
            Err(error) => {
                if error.code() == MF_E_ATTRIBUTENOTFOUND {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    }
}
