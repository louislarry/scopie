#![allow(dead_code, non_snake_case, non_camel_case_types)]
#[link(name = "ASICamera2")]
extern{}
/* automatically generated by rust-bindgen */

pub const ASICAMERA_ID_MAX: u32 = 128;
pub const ASI_BAYER_PATTERN_ASI_BAYER_RG: ASI_BAYER_PATTERN = 0;
pub const ASI_BAYER_PATTERN_ASI_BAYER_BG: ASI_BAYER_PATTERN = 1;
pub const ASI_BAYER_PATTERN_ASI_BAYER_GR: ASI_BAYER_PATTERN = 2;
pub const ASI_BAYER_PATTERN_ASI_BAYER_GB: ASI_BAYER_PATTERN = 3;
pub type ASI_BAYER_PATTERN = u32;
pub const ASI_IMG_TYPE_ASI_IMG_RAW8: ASI_IMG_TYPE = 0;
pub const ASI_IMG_TYPE_ASI_IMG_RGB24: ASI_IMG_TYPE = 1;
pub const ASI_IMG_TYPE_ASI_IMG_RAW16: ASI_IMG_TYPE = 2;
pub const ASI_IMG_TYPE_ASI_IMG_Y8: ASI_IMG_TYPE = 3;
pub const ASI_IMG_TYPE_ASI_IMG_END: ASI_IMG_TYPE = -1;
pub type ASI_IMG_TYPE = i32;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_NORTH: ASI_GUIDE_DIRECTION = 0;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_SOUTH: ASI_GUIDE_DIRECTION = 1;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_EAST: ASI_GUIDE_DIRECTION = 2;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_WEST: ASI_GUIDE_DIRECTION = 3;
pub type ASI_GUIDE_DIRECTION = u32;
pub const ASI_FLIP_STATUS_ASI_FLIP_NONE: ASI_FLIP_STATUS = 0;
pub const ASI_FLIP_STATUS_ASI_FLIP_HORIZ: ASI_FLIP_STATUS = 1;
pub const ASI_FLIP_STATUS_ASI_FLIP_VERT: ASI_FLIP_STATUS = 2;
pub const ASI_FLIP_STATUS_ASI_FLIP_BOTH: ASI_FLIP_STATUS = 3;
pub type ASI_FLIP_STATUS = u32;
pub const ASI_ERROR_CODE_ASI_SUCCESS: ASI_ERROR_CODE = 0;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX: ASI_ERROR_CODE = 1;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_ID: ASI_ERROR_CODE = 2;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE: ASI_ERROR_CODE = 3;
pub const ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED: ASI_ERROR_CODE = 4;
pub const ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED: ASI_ERROR_CODE = 5;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH: ASI_ERROR_CODE = 6;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT: ASI_ERROR_CODE = 7;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE: ASI_ERROR_CODE = 8;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE: ASI_ERROR_CODE = 9;
pub const ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY: ASI_ERROR_CODE = 10;
pub const ASI_ERROR_CODE_ASI_ERROR_TIMEOUT: ASI_ERROR_CODE = 11;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE: ASI_ERROR_CODE = 12;
pub const ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL: ASI_ERROR_CODE = 13;
pub const ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE: ASI_ERROR_CODE = 14;
pub const ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS: ASI_ERROR_CODE = 15;
pub const ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR: ASI_ERROR_CODE = 16;
pub const ASI_ERROR_CODE_ASI_ERROR_END: ASI_ERROR_CODE = 17;
pub type ASI_ERROR_CODE = u32;
pub const ASI_BOOL_ASI_FALSE: ASI_BOOL = 0;
pub const ASI_BOOL_ASI_TRUE: ASI_BOOL = 1;
pub type ASI_BOOL = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ASI_CAMERA_INFO {
    pub Name: [::std::os::raw::c_char; 64usize],
    pub CameraID: ::std::os::raw::c_int,
    pub MaxHeight: ::std::os::raw::c_long,
    pub MaxWidth: ::std::os::raw::c_long,
    pub IsColorCam: ASI_BOOL,
    pub BayerPattern: ASI_BAYER_PATTERN,
    pub SupportedBins: [::std::os::raw::c_int; 16usize],
    pub SupportedVideoFormat: [ASI_IMG_TYPE; 8usize],
    pub PixelSize: f64,
    pub MechanicalShutter: ASI_BOOL,
    pub ST4Port: ASI_BOOL,
    pub IsCoolerCam: ASI_BOOL,
    pub IsUSB3Host: ASI_BOOL,
    pub IsUSB3Camera: ASI_BOOL,
    pub ElecPerADU: f32,
    pub Unused: [::std::os::raw::c_char; 24usize],
}
#[test]
fn bindgen_test_layout__ASI_CAMERA_INFO() {
    assert_eq!(
        ::std::mem::size_of::<_ASI_CAMERA_INFO>(),
        248usize,
        concat!("Size of: ", stringify!(_ASI_CAMERA_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_CAMERA_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(_ASI_CAMERA_INFO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).Name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(Name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).CameraID as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(CameraID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).MaxHeight as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MaxHeight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).MaxWidth as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MaxWidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).IsColorCam as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsColorCam)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).BayerPattern as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(BayerPattern)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).SupportedBins as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(SupportedBins)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).SupportedVideoFormat as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(SupportedVideoFormat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).PixelSize as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(PixelSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).MechanicalShutter as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MechanicalShutter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).ST4Port as *const _ as usize },
        204usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(ST4Port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).IsCoolerCam as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsCoolerCam)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).IsUSB3Host as *const _ as usize },
        212usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsUSB3Host)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).IsUSB3Camera as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsUSB3Camera)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).ElecPerADU as *const _ as usize },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(ElecPerADU)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CAMERA_INFO>())).Unused as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(Unused)
        )
    );
}
pub type ASI_CAMERA_INFO = _ASI_CAMERA_INFO;
pub const ASI_CONTROL_TYPE_ASI_GAIN: ASI_CONTROL_TYPE = 0;
pub const ASI_CONTROL_TYPE_ASI_EXPOSURE: ASI_CONTROL_TYPE = 1;
pub const ASI_CONTROL_TYPE_ASI_GAMMA: ASI_CONTROL_TYPE = 2;
pub const ASI_CONTROL_TYPE_ASI_WB_R: ASI_CONTROL_TYPE = 3;
pub const ASI_CONTROL_TYPE_ASI_WB_B: ASI_CONTROL_TYPE = 4;
pub const ASI_CONTROL_TYPE_ASI_OFFSET: ASI_CONTROL_TYPE = 5;
pub const ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD: ASI_CONTROL_TYPE = 6;
pub const ASI_CONTROL_TYPE_ASI_OVERCLOCK: ASI_CONTROL_TYPE = 7;
pub const ASI_CONTROL_TYPE_ASI_TEMPERATURE: ASI_CONTROL_TYPE = 8;
pub const ASI_CONTROL_TYPE_ASI_FLIP: ASI_CONTROL_TYPE = 9;
pub const ASI_CONTROL_TYPE_ASI_AUTO_MAX_GAIN: ASI_CONTROL_TYPE = 10;
pub const ASI_CONTROL_TYPE_ASI_AUTO_MAX_EXP: ASI_CONTROL_TYPE = 11;
pub const ASI_CONTROL_TYPE_ASI_AUTO_MAX_BRIGHTNESS: ASI_CONTROL_TYPE = 12;
pub const ASI_CONTROL_TYPE_ASI_HARDWARE_BIN: ASI_CONTROL_TYPE = 13;
pub const ASI_CONTROL_TYPE_ASI_HIGH_SPEED_MODE: ASI_CONTROL_TYPE = 14;
pub const ASI_CONTROL_TYPE_ASI_COOLER_POWER_PERC: ASI_CONTROL_TYPE = 15;
pub const ASI_CONTROL_TYPE_ASI_TARGET_TEMP: ASI_CONTROL_TYPE = 16;
pub const ASI_CONTROL_TYPE_ASI_COOLER_ON: ASI_CONTROL_TYPE = 17;
pub const ASI_CONTROL_TYPE_ASI_MONO_BIN: ASI_CONTROL_TYPE = 18;
pub const ASI_CONTROL_TYPE_ASI_FAN_ON: ASI_CONTROL_TYPE = 19;
pub const ASI_CONTROL_TYPE_ASI_PATTERN_ADJUST: ASI_CONTROL_TYPE = 20;
pub const ASI_CONTROL_TYPE_ASI_ANTI_DEW_HEATER: ASI_CONTROL_TYPE = 21;
pub type ASI_CONTROL_TYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ASI_CONTROL_CAPS {
    pub Name: [::std::os::raw::c_char; 64usize],
    pub Description: [::std::os::raw::c_char; 128usize],
    pub MaxValue: ::std::os::raw::c_long,
    pub MinValue: ::std::os::raw::c_long,
    pub DefaultValue: ::std::os::raw::c_long,
    pub IsAutoSupported: ASI_BOOL,
    pub IsWritable: ASI_BOOL,
    pub ControlType: ASI_CONTROL_TYPE,
    pub Unused: [::std::os::raw::c_char; 32usize],
}
#[test]
fn bindgen_test_layout__ASI_CONTROL_CAPS() {
    assert_eq!(
        ::std::mem::size_of::<_ASI_CONTROL_CAPS>(),
        264usize,
        concat!("Size of: ", stringify!(_ASI_CONTROL_CAPS))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_CONTROL_CAPS>(),
        8usize,
        concat!("Alignment of ", stringify!(_ASI_CONTROL_CAPS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).Name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).Description as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Description)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).MaxValue as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(MaxValue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).MinValue as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(MinValue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).DefaultValue as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(DefaultValue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).IsAutoSupported as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(IsAutoSupported)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).IsWritable as *const _ as usize },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(IsWritable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).ControlType as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(ControlType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_CONTROL_CAPS>())).Unused as *const _ as usize },
        228usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Unused)
        )
    );
}
pub type ASI_CONTROL_CAPS = _ASI_CONTROL_CAPS;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_IDLE: ASI_EXPOSURE_STATUS = 0;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_WORKING: ASI_EXPOSURE_STATUS = 1;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS: ASI_EXPOSURE_STATUS = 2;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_FAILED: ASI_EXPOSURE_STATUS = 3;
pub type ASI_EXPOSURE_STATUS = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ASI_ID {
    pub id: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__ASI_ID() {
    assert_eq!(
        ::std::mem::size_of::<_ASI_ID>(),
        8usize,
        concat!("Size of: ", stringify!(_ASI_ID))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_ID>(),
        1usize,
        concat!("Alignment of ", stringify!(_ASI_ID))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ASI_ID>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_ID),
            "::",
            stringify!(id)
        )
    );
}
pub type ASI_ID = _ASI_ID;
extern "C" {
    /// Descriptions
    /// this should be the first API to be called
    /// get number of connected ASI cameras,
    ///
    /// Paras
    ///
    /// returnnumber of connected ASI cameras. 1 means 1 camera connected.
    pub fn ASIGetNumOfConnectedCameras() -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions:
    /// get the product ID of each supported camera, at first set pPIDs as 0 and get length and then malloc a buffer to contain the PIDs
    ///
    /// Paras:
    /// int* pPIDs: pointer to array of PIDs
    ///
    /// Return: length of the array.
    pub fn ASIGetProductIDs(pPIDs: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// get the property of the connected cameras, you can do this without open the camera.
    /// here is the sample code:
    ///
    /// int iNumofConnectCameras = ASIGetNumOfConnectedCameras();
    /// ASI_CAMERA_INFO **ppASICameraInfo = (ASI_CAMERA_INFO **)malloc(sizeof(ASI_CAMERA_INFO *)*iNumofConnectCameras);
    /// for(int i = 0; i < iNumofConnectCameras; i++)
    /// {
    /// ppASICameraInfo[i] = (ASI_CAMERA_INFO *)malloc(sizeof(ASI_CAMERA_INFO ));
    /// ASIGetCameraProperty(ppASICameraInfo[i], i);
    /// }
    ///
    /// Paras
    /// ASI_CAMERA_INFO *pASICameraInfo: Pointer to structure containing the property of camera
    /// user need to malloc the buffer
    /// int iCameraIndex: 0 means the first connect camera, 1 means the second connect camera
    ///
    /// return
    /// ASI_SUCCESS: Operation is successful
    /// ASI_ERROR_INVALID_INDEX  :no camera connected or index value out of boundary
    pub fn ASIGetCameraProperty(
        pASICameraInfo: *mut ASI_CAMERA_INFO,
        iCameraIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// open the camera before any operation to the camera, this will not affect the camera which is capturing
    /// All APIs below need to open the camera at first.
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return
    /// ASI_SUCCESS: Operation is successful
    /// ASI_ERROR_INVALID_ID  : no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_CAMERA_REMOVED: failed to find the camera, maybe camera has been removed
    pub fn ASIOpenCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    ///
    /// Initialise the camera after open, this function may take some while, this will affect the camera which is capturing
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIInitCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// you need to close the camera to free all the resource
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return
    /// ASI_SUCCESS :it will return success even the camera already closed
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASICloseCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get number of controls available for this camera. the camera need be opened at first.
    ///
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int * piNumberOfControls: pointer to an int to save the number of controls
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetNumOfControls(
        iCameraID: ::std::os::raw::c_int,
        piNumberOfControls: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get controls property available for this camera. the camera need be opened at first.
    /// user need to malloc and maintain the buffer.
    ///
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int iControlIndex: index of control, NOT control type
    /// ASI_CONTROL_CAPS * pControlCaps: Pointer to structure containing the property of the control
    /// user need to malloc the buffer
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetControlCaps(
        iCameraID: ::std::os::raw::c_int,
        iControlIndex: ::std::os::raw::c_int,
        pControlCaps: *mut ASI_CONTROL_CAPS,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get controls property value and auto value
    /// note:the value of the temperature is the float value * 10 to convert it to long type, control name is "Temperature"
    /// because long is the only type for control(except cooler's target temperature, because it is an integer)
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int ControlType: this is get from control property use the API ASIGetControlCaps
    /// long *plValue: pointer to the value you want to save the value get from control
    /// ASI_BOOL *pbAuto: pointer to the ASI_BOOL type
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_INVALID_CONTROL_TYPE, //invalid Control type
    pub fn ASIGetControlValue(
        iCameraID: ::std::os::raw::c_int,
        ControlType: ::std::os::raw::c_int,
        plValue: *mut ::std::os::raw::c_long,
        pbAuto: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Set controls property value and auto value
    /// it will return success and set the max value or min value if the value is beyond the boundary
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int ControlType: this is get from control property use the API ASIGetControlCaps
    /// long lValue: the value set to the control
    /// ASI_BOOL bAuto: set the control auto
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_INVALID_CONTROL_TYPE, //invalid Control type
    /// ASI_ERROR_GENERAL_ERROR,//general error, eg: value is out of valid range; operate to camera hareware failed
    pub fn ASISetControlValue(
        iCameraID: ::std::os::raw::c_int,
        ControlType: ::std::os::raw::c_int,
        lValue: ::std::os::raw::c_long,
        bAuto: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// set the ROI area before capture.
    /// you must stop capture before call it.
    /// the width and height is the value after binning.
    /// ie. you need to set width to 640 and height to 480 if you want to run at 640X480@BIN2
    /// ASI120's data size must be times of 1024 which means width*height%1024=0
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int iWidth,  the width of the ROI area. Make sure iWidth%8 = 0.
    /// int iHeight,  the height of the ROI area. Make sure iHeight%2 = 0,
    /// further, for USB2.0 camera ASI120, please make sure that iWidth*iHeight%1024=0.
    /// int iBin,   binning method. bin1=1, bin2=2
    /// ASI_IMG_TYPE Img_type: the output format you want
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_INVALID_SIZE, //wrong video format size
    /// ASI_ERROR_INVALID_IMGTYPE, //unsupported image format, make sure iWidth and iHeight and binning is set correct
    pub fn ASISetROIFormat(
        iCameraID: ::std::os::raw::c_int,
        iWidth: ::std::os::raw::c_int,
        iHeight: ::std::os::raw::c_int,
        iBin: ::std::os::raw::c_int,
        Img_type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get the current ROI area setting .
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int *piWidth,  pointer to the width of the ROI area
    /// int *piHeight, pointer to the height of the ROI area.
    /// int *piBin,   pointer to binning method. bin1=1, bin2=2
    /// ASI_IMG_TYPE *pImg_type: pointer to the output format
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetROIFormat(
        iCameraID: ::std::os::raw::c_int,
        piWidth: *mut ::std::os::raw::c_int,
        piHeight: *mut ::std::os::raw::c_int,
        piBin: *mut ::std::os::raw::c_int,
        pImg_type: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Set the start position of the ROI area.
    /// you can call this API to move the ROI area when video is streaming
    /// the camera will set the ROI area to the center of the full image as default
    /// at bin2 or bin3 mode, the position is relative to the image after binning
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int iStartX, pointer to the start X
    /// int iStartY  pointer to the start Y
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_OUTOF_BOUNDARY: the start x and start y make the image out of boundary
    pub fn ASISetStartPos(
        iCameraID: ::std::os::raw::c_int,
        iStartX: ::std::os::raw::c_int,
        iStartY: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get the start position of current ROI area .
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int *piStartX, pointer to the start X
    /// int *piStartY  pointer to the start Y
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetStartPos(
        iCameraID: ::std::os::raw::c_int,
        piStartX: *mut ::std::os::raw::c_int,
        piStartY: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Get the droped frames .
    /// drop frames happen when USB is traffic or harddisk write speed is slow
    /// it will reset to 0 after stop capture
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// int *piDropFrames pointer to drop frames
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetDroppedFrames(
        iCameraID: ::std::os::raw::c_int,
        piDropFrames: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// provide a dark file's path to the function and enable dark subtract
    /// this is used when there is hot pixel or need to do long exposure
    /// you'd better make this dark file from the  "dark subtract" funtion
    /// of the "video capture filter" directshow page.
    /// the dark file's size should be the same of camera's max width and height
    /// and should be RGB8 raw format.it will on even you changed the ROI setting
    /// it only correct the hot pixels if out put isn't 16bit.
    ///
    /// it will be remembered in registry. so "Dark subtract" is on next time if you close your app.
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// char *pcBMPPath: the path to the bmp dark file.
    /// return
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_PATH, //cannot find the path of the file
    /// ASI_ERROR_INVALID_FILEFORMAT, //the dark file's size should be the same of camera's max width and height
    pub fn ASIEnableDarkSubtract(
        iCameraID: ::std::os::raw::c_int,
        pcBMPPath: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Disable the dark subtract function.
    /// you'd better call it at start if you don't want to use it.
    /// because dark subtract function is remembered on windows platform
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    pub fn ASIDisableDarkSubtract(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Start video capture
    /// then you can get the data from the API ASIGetVideoData
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful, it will return success if already started
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_EXPOSURE_IN_PROGRESS: snap mode is working, you need to stop snap first
    pub fn ASIStartVideoCapture(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Stop video capture
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful, it will return success if already stopped
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIStopVideoCapture(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// get data from the video buffer.the buffer is very small
    /// you need to call this API as fast as possible, otherwise frame will be discarded
    /// so the best way is maintain one buffer loop and call this API in a loop
    /// please make sure the buffer size is biger enough to hold one image
    /// otherwise the this API will crash
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// unsigned char* pBuffer, caller need to malloc the buffer, make sure the size is big enough
    /// the size in byte:
    /// 8bit mono:width*height
    /// 16bit mono:width*height*2
    /// RGB24:width*height*3
    ///
    /// int iWaitms, this API will block and wait iWaitms to get one image. the unit is ms
    /// -1 means wait forever. this value is recommend set to exposure*2+500ms
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_TIMEOUT: no image get and timeout
    pub fn ASIGetVideoData(
        iCameraID: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_uchar,
        lBuffSize: ::std::os::raw::c_long,
        iWaitms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// PulseGuide of the ST4 port on. this function only work on the module which have ST4 port
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_GUIDE_DIRECTION direction the direction of guider
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIPulseGuideOn(
        iCameraID: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// PulseGuide of the ST4 port off. this function only work on the module which have ST4 port
    /// make sure where is ASIPulseGuideOn and there is ASIPulseGuideOff
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_GUIDE_DIRECTION direction the direction of guider
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIPulseGuideOff(
        iCameraID: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// Start camera exposure. the following 4 API is usually used when long exposure required
    /// start exposure  and check the exposure status then get the data
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_BOOL bIsDark: means dark frame if there is mechanical shutter on the camera. otherwise useless
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_VIDEO_MODE_ACTIVE: video mode is working, you need to stop video capture first
    pub fn ASIStartExposure(
        iCameraID: ::std::os::raw::c_int,
        bIsDark: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// to cancel the long exposure which is on.
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    ///
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIStopExposure(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// to get the exposure status, work with ASIStartExposure.
    /// you can read the data if get ASI_EXP_SUCCESS. or have to restart exposure again
    /// if get ASI_EXP_FAILED
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_EXPOSURE_STATUS *pExpStatus: the exposure status
    ///
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetExpStatus(
        iCameraID: ::std::os::raw::c_int,
        pExpStatus: *mut ASI_EXPOSURE_STATUS,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// get data after exposure.
    /// please make sure the buffer size is biger enough to hold one image
    /// otherwise the this API will crash
    ///
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// unsigned char* pBuffer, caller need to malloc the buffer, make sure the size is big enough
    /// the size in byte:
    /// 8bit mono:width*height
    /// 16bit mono:width*height*2
    /// RGB24:width*height*3
    ///
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    /// ASI_ERROR_TIMEOUT: no image get and timeout
    pub fn ASIGetDataAfterExp(
        iCameraID: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_uchar,
        lBuffSize: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// get camera id stored in flash, only available for USB3.0 camera
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_ID* pID: pointer to ID
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetID(iCameraID: ::std::os::raw::c_int, pID: *mut ASI_ID) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// write camera id to flash, only available for USB3.0 camera
    ///
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// ASI_ID ID: ID
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASISetID(iCameraID: ::std::os::raw::c_int, ID: ASI_ID) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Descriptions
    /// get pre-setting parameter
    /// Paras
    /// int CameraID: this is get from the camera property use the API ASIGetCameraProperty
    /// Offset_HighestDR: offset at highest dynamic range,
    /// Offset_UnityGain: offset at unity gain
    /// int *Gain_LowestRN, *Offset_LowestRN: gain and offset at lowest read noise
    ///
    /// return:
    /// ASI_SUCCESS : Operation is successful
    /// ASI_ERROR_CAMERA_CLOSED : camera didn't open
    /// ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
    pub fn ASIGetGainOffset(
        iCameraID: ::std::os::raw::c_int,
        pOffset_HighestDR: *mut ::std::os::raw::c_int,
        pOffset_UnityGain: *mut ::std::os::raw::c_int,
        pGain_LowestRN: *mut ::std::os::raw::c_int,
        pOffset_LowestRN: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
