mod workaround;

use windows::{
    core::{implement, ComInterface, Result, GUID},
    Media::Core::{IMediaSource, IMediaSource_Impl},
    Win32::{
        Foundation::{BOOL, E_NOINTERFACE},
        Media::MediaFoundation::{
            IMFAttributes, IMFGetService, IMFGetService_Impl, IMFMediaSource, IMFMediaType,
            MF_E_ATTRIBUTENOTFOUND, MF_E_UNSUPPORTED_SERVICE, MF_MEDIASOURCE_SERVICE,
            MF_MT_FRAME_RATE, MF_MT_FRAME_RATE_RANGE_MAX, MF_MT_FRAME_RATE_RANGE_MIN,
            MF_MT_FRAME_SIZE, MF_MT_GEOMETRIC_APERTURE, MF_MT_MINIMUM_DISPLAY_APERTURE,
            MF_MT_PAN_SCAN_APERTURE, MF_MT_PIXEL_ASPECT_RATIO,
        },
        System::{
            Com::StructuredStorage::{PropVariantClear, PROPVARIANT},
            Variant::{VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR},
        },
    },
};

use self::workaround::get_guid_name_const;

#[implement(IMediaSource, IMFGetService)]
pub struct CustomSource(pub IMFMediaSource);

impl IMediaSource_Impl for CustomSource {}
#[allow(non_snake_case)]
impl IMFGetService_Impl for CustomSource {
    fn GetService(
        &self,
        guidservice: *const GUID,
        riid: *const GUID,
        ppvobject: *mut *mut std::ffi::c_void,
    ) -> Result<()> {
        let service_guid = unsafe { *guidservice };
        if service_guid == MF_MEDIASOURCE_SERVICE {
            let riid = unsafe { *riid };
            if riid == IMFMediaSource::IID {
                unsafe { *ppvobject = std::mem::transmute(self.0.clone()) };
            } else {
                return Err(E_NOINTERFACE.into());
            }
        } else {
            return Err(MF_E_UNSUPPORTED_SERVICE.into());
        }
        Ok(())
    }
}

// https://learn.microsoft.com/en-us/windows/win32/medfound/how-to-set-the-video-capture-format
pub fn enum_formats(media_source: &IMFMediaSource) -> Result<()> {
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

// https://learn.microsoft.com/en-us/windows/win32/medfound/media-type-debugging-code
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

fn get_guid_name(guid: GUID) -> String {
    if let Some(guid_name) = get_guid_name_const(guid) {
        guid_name.to_owned()
    } else {
        format!("{:?}", guid)
    }
}

pub fn get_string_attribute(
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
