mod app;
mod dispatcher_queue;
mod hotkey;
mod media;
mod window;

use std::io::Write;

use dispatcher_queue::{
    create_dispatcher_queue_controller_for_current_thread,
    shutdown_dispatcher_queue_controller_and_wait,
};
use hotkey::HotKey;
use media::get_string_attribute;
use windows::{
    Foundation::{Size, TypedEventHandler},
    Media::{
        Core::{IMediaSource, MediaSource},
        Playback::{MediaPlaybackItem, MediaPlayer},
    },
    UI::{Color, Composition::Compositor},
    Win32::{
        Media::MediaFoundation::{
            IMFActivate, IMFMediaSource, MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
            MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
            MF_VERSION, MFCreateAttributes, MFEnumDeviceSources, MFSTARTUP_FULL, MFStartup,
        },
        System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize},
        UI::{
            Input::KeyboardAndMouse::{MOD_ALT, VK_RETURN},
            WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, TranslateMessage},
        },
    },
    core::{Array, IInspectable, Interface, Result},
};
use windows_numerics::Vector2;

use crate::{
    app::App,
    media::{CustomSource, enum_formats},
    window::Window,
};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let debug_output = args.contains(&"--debug".to_owned());

    unsafe {
        RoInitialize(RO_INIT_SINGLETHREADED)?;
    }
    unsafe { MFStartup(MF_VERSION, MFSTARTUP_FULL)? }

    let controller = create_dispatcher_queue_controller_for_current_thread()?;
    let compositor = Compositor::new()?;

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

    let mut message = MSG::default();

    if sources.is_empty() {
        println!("No video capture devices found!");
    } else {
        if let Some(source) = select_source(&sources)? {
            let display_name = get_friendly_name(&source)?.unwrap_or("Unknown".to_owned());
            println!("Using {}...", display_name);

            // Activate the media source
            let media_source: IMFMediaSource = unsafe { source.ActivateObject()? };

            // Enumerate supported formats
            if debug_output {
                enum_formats(&media_source)?;
            }

            let custom_source: IMediaSource = CustomSource(media_source).into();
            let winrt_media_source = MediaSource::CreateFromIMediaSource(&custom_source)?;
            let item = MediaPlaybackItem::Create(&winrt_media_source)?;

            let media_player = MediaPlayer::new()?;
            let media_surface = media_player.GetSurface(&compositor)?;
            media_player.SourceChanged(&TypedEventHandler::<MediaPlayer, IInspectable>::new(
                move |player, _| -> Result<()> {
                    let player = player.as_ref().unwrap();
                    let item: MediaPlaybackItem = player.Source()?.cast()?;
                    let size = get_max_video_track_size(&item)?;
                    player.SetSurfaceSize(size)?;
                    Ok(())
                },
            ))?;

            media_player.SetSource(&item)?;

            let root = compositor.CreateSpriteVisual()?;
            root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
            root.SetBrush(&compositor.CreateColorBrushWithColor(Color {
                A: 255,
                R: 0,
                G: 0,
                B: 0,
            })?)?;

            let content_visual = compositor.CreateSpriteVisual()?;
            content_visual.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
            let content_brush = compositor.CreateSurfaceBrush()?;
            content_brush.SetSurface(&media_surface.CompositionSurface()?)?;
            content_visual.SetBrush(&content_brush)?;
            root.Children()?.InsertAtTop(&content_visual)?;

            let app = App::new();
            let window = Window::new(&display_name, 800, 600, app)?;
            let target = window.create_window_target(&compositor, false)?;
            target.SetRoot(&root)?;

            media_player.Play()?;

            let _hot_key = HotKey::new(window.handle(), MOD_ALT, VK_RETURN.0 as u32)?;

            unsafe {
                while GetMessageW(&mut message, None, 0, 0).into() {
                    let _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        } else {
            // Do nothing
        }
    }

    let _result =
        shutdown_dispatcher_queue_controller_and_wait(&controller, message.wParam.0 as i32)?;

    Ok(())
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

fn get_friendly_name<T: Interface>(activate: &T) -> Result<Option<String>> {
    get_string_attribute(&activate.cast()?, &MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME)
}

fn get_max_video_track_size(item: &MediaPlaybackItem) -> Result<Size> {
    let tracks = item.VideoTracks()?;
    let mut size = Size::default();
    for track in tracks {
        let properties = track.GetEncodingProperties()?;
        let width = properties.Width()? as f32;
        let height = properties.Height()? as f32;
        if size.Width < width {
            size.Width = width;
        }
        if size.Height < height {
            size.Height = height;
        }
    }
    Ok(size)
}
