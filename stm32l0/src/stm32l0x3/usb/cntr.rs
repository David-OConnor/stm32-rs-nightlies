#[doc = "Register `CNTR` reader"]
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR` writer"]
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRES` reader - FRES"]
pub type FRES_R = crate::BitReader<FRES_A>;
#[doc = "FRES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRES_A {
    #[doc = "0: Clear USB reset"]
    NoReset = 0,
    #[doc = "1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    Reset = 1,
}
impl From<FRES_A> for bool {
    #[inline(always)]
    fn from(variant: FRES_A) -> Self {
        variant as u8 != 0
    }
}
impl FRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRES_A {
        match self.bits {
            false => FRES_A::NoReset,
            true => FRES_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FRES_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FRES_A::Reset
    }
}
#[doc = "Field `FRES` writer - FRES"]
pub type FRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, FRES_A, O>;
impl<'a, const O: u8> FRES_W<'a, O> {
    #[doc = "Clear USB reset"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FRES_A::NoReset)
    }
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRES_A::Reset)
    }
}
#[doc = "Field `PDWN` reader - PDWN"]
pub type PDWN_R = crate::BitReader<PDWN_A>;
#[doc = "PDWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDWN_A {
    #[doc = "0: No power down"]
    Disabled = 0,
    #[doc = "1: Enter power down mode"]
    Enabled = 1,
}
impl From<PDWN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWN_A) -> Self {
        variant as u8 != 0
    }
}
impl PDWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWN_A {
        match self.bits {
            false => PDWN_A::Disabled,
            true => PDWN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDWN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDWN_A::Enabled
    }
}
#[doc = "Field `PDWN` writer - PDWN"]
pub type PDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, PDWN_A, O>;
impl<'a, const O: u8> PDWN_W<'a, O> {
    #[doc = "No power down"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDWN_A::Disabled)
    }
    #[doc = "Enter power down mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDWN_A::Enabled)
    }
}
#[doc = "Field `LPMODE` reader - LPMODE"]
pub type LPMODE_R = crate::BitReader<LPMODE_A>;
#[doc = "LPMODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMODE_A {
    #[doc = "0: No low-power mode"]
    Disabled = 0,
    #[doc = "1: Enter low-power mode"]
    Enabled = 1,
}
impl From<LPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            false => LPMODE_A::Disabled,
            true => LPMODE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMODE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMODE_A::Enabled
    }
}
#[doc = "Field `LPMODE` writer - LPMODE"]
pub type LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, LPMODE_A, O>;
impl<'a, const O: u8> LPMODE_W<'a, O> {
    #[doc = "No low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMODE_A::Disabled)
    }
    #[doc = "Enter low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMODE_A::Enabled)
    }
}
#[doc = "Field `FSUSP` reader - FSUSP"]
pub type FSUSP_R = crate::BitReader<FSUSP_A>;
#[doc = "FSUSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSUSP_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    Suspend = 1,
}
impl From<FSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl FSUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSUSP_A {
        match self.bits {
            false => FSUSP_A::NoEffect,
            true => FSUSP_A::Suspend,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FSUSP_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Suspend`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSUSP_A::Suspend
    }
}
#[doc = "Field `FSUSP` writer - FSUSP"]
pub type FSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, FSUSP_A, O>;
impl<'a, const O: u8> FSUSP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FSUSP_A::NoEffect)
    }
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(FSUSP_A::Suspend)
    }
}
#[doc = "Field `RESUME` reader - RESUME"]
pub type RESUME_R = crate::BitReader<RESUME_A>;
#[doc = "RESUME\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_A {
    #[doc = "1: Resume requested"]
    Requested = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESUME_A> {
        match self.bits {
            true => Some(RESUME_A::Requested),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Requested`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == RESUME_A::Requested
    }
}
#[doc = "Field `RESUME` writer - RESUME"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, RESUME_A, O>;
impl<'a, const O: u8> RESUME_W<'a, O> {
    #[doc = "Resume requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(RESUME_A::Requested)
    }
}
#[doc = "Field `L1RESUME` reader - L1RESUME"]
pub type L1RESUME_R = crate::BitReader<L1RESUME_A>;
#[doc = "L1RESUME\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RESUME_A {
    #[doc = "1: LPM L1 request requested"]
    Requested = 1,
}
impl From<L1RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: L1RESUME_A) -> Self {
        variant as u8 != 0
    }
}
impl L1RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<L1RESUME_A> {
        match self.bits {
            true => Some(L1RESUME_A::Requested),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Requested`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == L1RESUME_A::Requested
    }
}
#[doc = "Field `L1RESUME` writer - L1RESUME"]
pub type L1RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, L1RESUME_A, O>;
impl<'a, const O: u8> L1RESUME_W<'a, O> {
    #[doc = "LPM L1 request requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(L1RESUME_A::Requested)
    }
}
#[doc = "Field `L1REQM` reader - L1REQM"]
pub type L1REQM_R = crate::BitReader<L1REQM_A>;
#[doc = "L1REQM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQM_A {
    #[doc = "0: L1REQ Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<L1REQM_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQM_A) -> Self {
        variant as u8 != 0
    }
}
impl L1REQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1REQM_A {
        match self.bits {
            false => L1REQM_A::Disabled,
            true => L1REQM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == L1REQM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == L1REQM_A::Enabled
    }
}
#[doc = "Field `L1REQM` writer - L1REQM"]
pub type L1REQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, L1REQM_A, O>;
impl<'a, const O: u8> L1REQM_W<'a, O> {
    #[doc = "L1REQ Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(L1REQM_A::Disabled)
    }
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(L1REQM_A::Enabled)
    }
}
#[doc = "Field `ESOFM` reader - ESOFM"]
pub type ESOFM_R = crate::BitReader<ESOFM_A>;
#[doc = "ESOFM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFM_A {
    #[doc = "0: ESOF Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<ESOFM_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFM_A) -> Self {
        variant as u8 != 0
    }
}
impl ESOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESOFM_A {
        match self.bits {
            false => ESOFM_A::Disabled,
            true => ESOFM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESOFM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESOFM_A::Enabled
    }
}
#[doc = "Field `ESOFM` writer - ESOFM"]
pub type ESOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, ESOFM_A, O>;
impl<'a, const O: u8> ESOFM_W<'a, O> {
    #[doc = "ESOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESOFM_A::Disabled)
    }
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESOFM_A::Enabled)
    }
}
#[doc = "Field `SOFM` reader - SOFM"]
pub type SOFM_R = crate::BitReader<SOFM_A>;
#[doc = "SOFM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFM_A {
    #[doc = "0: SOF Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<SOFM_A> for bool {
    #[inline(always)]
    fn from(variant: SOFM_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFM_A {
        match self.bits {
            false => SOFM_A::Disabled,
            true => SOFM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOFM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOFM_A::Enabled
    }
}
#[doc = "Field `SOFM` writer - SOFM"]
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, SOFM_A, O>;
impl<'a, const O: u8> SOFM_W<'a, O> {
    #[doc = "SOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOFM_A::Disabled)
    }
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOFM_A::Enabled)
    }
}
#[doc = "Field `RESETM` reader - RESETM"]
pub type RESETM_R = crate::BitReader<RESETM_A>;
#[doc = "RESETM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETM_A {
    #[doc = "0: RESET Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<RESETM_A> for bool {
    #[inline(always)]
    fn from(variant: RESETM_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETM_A {
        match self.bits {
            false => RESETM_A::Disabled,
            true => RESETM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETM_A::Enabled
    }
}
#[doc = "Field `RESETM` writer - RESETM"]
pub type RESETM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, RESETM_A, O>;
impl<'a, const O: u8> RESETM_W<'a, O> {
    #[doc = "RESET Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETM_A::Disabled)
    }
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETM_A::Enabled)
    }
}
#[doc = "Field `SUSPM` reader - SUSPM"]
pub type SUSPM_R = crate::BitReader<SUSPM_A>;
#[doc = "SUSPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPM_A {
    #[doc = "0: Suspend Mode Request SUSP Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<SUSPM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPM_A {
        match self.bits {
            false => SUSPM_A::Disabled,
            true => SUSPM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPM_A::Enabled
    }
}
#[doc = "Field `SUSPM` writer - SUSPM"]
pub type SUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, SUSPM_A, O>;
impl<'a, const O: u8> SUSPM_W<'a, O> {
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUSPM_A::Disabled)
    }
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUSPM_A::Enabled)
    }
}
#[doc = "Field `WKUPM` reader - WKUPM"]
pub type WKUPM_R = crate::BitReader<WKUPM_A>;
#[doc = "WKUPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPM_A {
    #[doc = "0: WKUP Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<WKUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPM_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPM_A {
        match self.bits {
            false => WKUPM_A::Disabled,
            true => WKUPM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUPM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUPM_A::Enabled
    }
}
#[doc = "Field `WKUPM` writer - WKUPM"]
pub type WKUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, WKUPM_A, O>;
impl<'a, const O: u8> WKUPM_W<'a, O> {
    #[doc = "WKUP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUPM_A::Disabled)
    }
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUPM_A::Enabled)
    }
}
#[doc = "Field `ERRM` reader - ERRM"]
pub type ERRM_R = crate::BitReader<ERRM_A>;
#[doc = "ERRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRM_A {
    #[doc = "0: ERR Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<ERRM_A> for bool {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRM_A {
        match self.bits {
            false => ERRM_A::Disabled,
            true => ERRM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRM_A::Enabled
    }
}
#[doc = "Field `ERRM` writer - ERRM"]
pub type ERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, ERRM_A, O>;
impl<'a, const O: u8> ERRM_W<'a, O> {
    #[doc = "ERR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRM_A::Disabled)
    }
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRM_A::Enabled)
    }
}
#[doc = "Field `PMAOVRM` reader - PMAOVRM"]
pub type PMAOVRM_R = crate::BitReader<PMAOVRM_A>;
#[doc = "PMAOVRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRM_A {
    #[doc = "0: PMAOVR Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<PMAOVRM_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM_A) -> Self {
        variant as u8 != 0
    }
}
impl PMAOVRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAOVRM_A {
        match self.bits {
            false => PMAOVRM_A::Disabled,
            true => PMAOVRM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PMAOVRM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PMAOVRM_A::Enabled
    }
}
#[doc = "Field `PMAOVRM` writer - PMAOVRM"]
pub type PMAOVRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, PMAOVRM_A, O>;
impl<'a, const O: u8> PMAOVRM_W<'a, O> {
    #[doc = "PMAOVR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::Disabled)
    }
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::Enabled)
    }
}
#[doc = "Field `CTRM` reader - CTRM"]
pub type CTRM_R = crate::BitReader<CTRM_A>;
#[doc = "CTRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRM_A {
    #[doc = "0: Correct Transfer (CTR) Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<CTRM_A> for bool {
    #[inline(always)]
    fn from(variant: CTRM_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRM_A {
        match self.bits {
            false => CTRM_A::Disabled,
            true => CTRM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTRM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTRM_A::Enabled
    }
}
#[doc = "Field `CTRM` writer - CTRM"]
pub type CTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, CTRM_A, O>;
impl<'a, const O: u8> CTRM_W<'a, O> {
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTRM_A::Disabled)
    }
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTRM_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<0> {
        FRES_W::new(self)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<1> {
        PDWN_W::new(self)
    }
    #[doc = "Bit 2 - LPMODE"]
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LPMODE_W<2> {
        LPMODE_W::new(self)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn fsusp(&mut self) -> FSUSP_W<3> {
        FSUSP_W::new(self)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<4> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<5> {
        L1RESUME_W::new(self)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<7> {
        L1REQM_W::new(self)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<8> {
        ESOFM_W::new(self)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<9> {
        SOFM_W::new(self)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    #[must_use]
    pub fn resetm(&mut self) -> RESETM_W<10> {
        RESETM_W::new(self)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<11> {
        SUSPM_W::new(self)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<12> {
        WKUPM_W::new(self)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<13> {
        ERRM_W::new(self)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<14> {
        PMAOVRM_W::new(self)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<15> {
        CTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](index.html) module"]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr::R](R) reader structure"]
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr::W](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
