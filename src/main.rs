use windows::{core::{Result, Array, GUID, ComInterface}, Win32::{System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize}, Media::MediaFoundation::{MFStartup, MF_VERSION, MFSTARTUP_FULL, MFCreateAttributes, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID, MFEnumDeviceSources, IMFActivate, IMFAttributes, MF_E_ATTRIBUTENOTFOUND, MFT_FRIENDLY_NAME_Attribute, IMFMediaSource, MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME}}};

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
        attributes.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID)?;
    }

    let sources = unsafe {
        let mut data = std::ptr::null_mut();
        let mut len = 0;
        MFEnumDeviceSources(&attributes, &mut data, &mut len)?;
        Array::<IMFActivate>::from_raw_parts(data as _, len)
    };

    for source in sources.as_slice() {
        let source = source.as_ref().unwrap();
        //let source: IMFMediaSource = unsafe { source.ActivateObject()? };
        let display_name = get_friendly_name(source)?.unwrap_or("Unknown".to_owned());
        println!("{}", display_name);
    }

    Ok(())
}

fn get_friendly_name<T: ComInterface>(
    activate: &T
) -> Result<Option<String>> {
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