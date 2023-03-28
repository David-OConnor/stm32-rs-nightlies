#[doc = "Register `USB_CNTR` reader"]
pub struct R(crate::R<USB_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CNTR` writer"]
pub struct W(crate::W<USB_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USB_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRST` reader - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `PDWN` reader - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_R = crate::BitReader<bool>;
#[doc = "Field `PDWN` writer - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `SUSPRDY` reader - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
pub type SUSPRDY_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEN` reader - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEN` writer - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `L2RES` reader - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_R = crate::BitReader<bool>;
#[doc = "Field `L2RES` writer - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `L1RES` reader - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 μs remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
pub type L1RES_R = crate::BitReader<bool>;
#[doc = "Field `L1RES` writer - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 μs remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
pub type L1RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1REQM_R = crate::BitReader<bool>;
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1REQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader<bool>;
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader<bool>;
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `RST_DCONM` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_R = crate::BitReader<bool>;
#[doc = "Field `RST_DCONM` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader<bool>;
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader<bool>;
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader<bool>;
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader<bool>;
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader<bool>;
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `THR512M` reader - 512 byte threshold interrupt mask"]
pub type THR512M_R = crate::BitReader<bool>;
#[doc = "Field `THR512M` writer - 512 byte threshold interrupt mask"]
pub type THR512M_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `DDISCM` reader - Device disconnection mask Host mode"]
pub type DDISCM_R = crate::BitReader<bool>;
#[doc = "Field `DDISCM` writer - Device disconnection mask Host mode"]
pub type DDISCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
#[doc = "Field `HOST` reader - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_R = crate::BitReader<bool>;
#[doc = "Field `HOST` writer - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CNTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
    #[inline(always)]
    pub fn susprdy(&self) -> SUSPRDY_R {
        SUSPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    pub fn l2res(&self) -> L2RES_R {
        L2RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 μs remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn l1res(&self) -> L1RES_R {
        L1RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    pub fn rst_dconm(&self) -> RST_DCONM_R {
        RST_DCONM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    pub fn thr512m(&self) -> THR512M_R {
        THR512M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device disconnection mask Host mode"]
    #[inline(always)]
    pub fn ddiscm(&self) -> DDISCM_R {
        DDISCM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<0> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<1> {
        PDWN_W::new(self)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    #[must_use]
    pub fn suspen(&mut self) -> SUSPEN_W<3> {
        SUSPEN_W::new(self)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l2res(&mut self) -> L2RES_W<4> {
        L2RES_W::new(self)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 μs remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn l1res(&mut self) -> L1RES_W<5> {
        L1RES_W::new(self)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<7> {
        L1REQM_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<8> {
        ESOFM_W::new(self)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<9> {
        SOFM_W::new(self)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rst_dconm(&mut self) -> RST_DCONM_W<10> {
        RST_DCONM_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<11> {
        SUSPM_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<12> {
        WKUPM_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<13> {
        ERRM_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<14> {
        PMAOVRM_W::new(self)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<15> {
        CTRM_W::new(self)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn thr512m(&mut self) -> THR512M_W<16> {
        THR512M_W::new(self)
    }
    #[doc = "Bit 17 - Device disconnection mask Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscm(&mut self) -> DDISCM_W<17> {
        DDISCM_W::new(self)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HOST_W<31> {
        HOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_cntr](index.html) module"]
pub struct USB_CNTR_SPEC;
impl crate::RegisterSpec for USB_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_cntr::R](R) reader structure"]
impl crate::Readable for USB_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_cntr::W](W) writer structure"]
impl crate::Writable for USB_CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CNTR to value 0x03"]
impl crate::Resettable for USB_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
