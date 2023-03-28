#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMS` reader - Low-power mode selection"]
pub type LPMS_R = crate::FieldReader<u8, LPMS_A>;
#[doc = "Low-power mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS_A {
    #[doc = "0: Stop 0 mode"]
    Stop0 = 0,
    #[doc = "1: Stop 1 mode"]
    Stop1 = 1,
    #[doc = "2: Stop 2 mode"]
    Stop2 = 2,
    #[doc = "3: Standby mode"]
    Standby = 3,
    #[doc = "4: Shutdown mode"]
    Shutdown = 4,
}
impl From<LPMS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMS_A) -> Self {
        variant as _
    }
}
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMS_A> {
        match self.bits {
            0 => Some(LPMS_A::Stop0),
            1 => Some(LPMS_A::Stop1),
            2 => Some(LPMS_A::Stop2),
            3 => Some(LPMS_A::Standby),
            4 => Some(LPMS_A::Shutdown),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Stop0`"]
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS_A::Stop0
    }
    #[doc = "Checks if the value of the field is `Stop1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS_A::Stop1
    }
    #[doc = "Checks if the value of the field is `Stop2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS_A::Stop2
    }
    #[doc = "Checks if the value of the field is `Standby`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS_A::Standby
    }
    #[doc = "Checks if the value of the field is `Shutdown`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS_A::Shutdown
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection"]
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, LPMS_A, 3, O>;
impl<'a, const O: u8> LPMS_W<'a, O> {
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn stop0(self) -> &'a mut W {
        self.variant(LPMS_A::Stop0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(LPMS_A::Stop1)
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(LPMS_A::Stop2)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMS_A::Standby)
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(LPMS_A::Shutdown)
    }
}
#[doc = "Field `RRSTP` reader - SRAM3 retention in Stop 2 mode"]
pub type RRSTP_R = crate::BitReader<RRSTP_A>;
#[doc = "SRAM3 retention in Stop 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRSTP_A {
    #[doc = "0: SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)"]
    Disabled = 0,
    #[doc = "1: SRAM3 is powered in Stop 2 mode (RAM3 content is kept)"]
    Enabled = 1,
}
impl From<RRSTP_A> for bool {
    #[inline(always)]
    fn from(variant: RRSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl RRSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRSTP_A {
        match self.bits {
            false => RRSTP_A::Disabled,
            true => RRSTP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRSTP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRSTP_A::Enabled
    }
}
#[doc = "Field `RRSTP` writer - SRAM3 retention in Stop 2 mode"]
pub type RRSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RRSTP_A, O>;
impl<'a, const O: u8> RRSTP_W<'a, O> {
    #[doc = "SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RRSTP_A::Disabled)
    }
    #[doc = "SRAM3 is powered in Stop 2 mode (RAM3 content is kept)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RRSTP_A::Enabled)
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<DBP_A>;
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP_A {
    #[doc = "0: Access to RTC and Backup registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to RTC and Backup registers enabled"]
    Enabled = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::Disabled,
            true => DBP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP_A::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DBP_A, O>;
impl<'a, const O: u8> DBP_W<'a, O> {
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::Disabled)
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBP_A::Enabled)
    }
}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<u8, VOS_A>;
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "1: Range 1"]
    Range1 = 1,
    #[doc = "2: Range 1"]
    Range2 = 2,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOS_A> {
        match self.bits {
            1 => Some(VOS_A::Range1),
            2 => Some(VOS_A::Range2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Range1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == VOS_A::Range1
    }
    #[doc = "Checks if the value of the field is `Range2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == VOS_A::Range2
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, VOS_A, 2, O>;
impl<'a, const O: u8> VOS_W<'a, O> {
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(VOS_A::Range1)
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(VOS_A::Range2)
    }
}
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader<LPR_A>;
#[doc = "Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR_A {
    #[doc = "0: Main Mode"]
    MainMode = 0,
    #[doc = "1: Low Power Mode"]
    LowPowerMode = 1,
}
impl From<LPR_A> for bool {
    #[inline(always)]
    fn from(variant: LPR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPR_A {
        match self.bits {
            false => LPR_A::MainMode,
            true => LPR_A::LowPowerMode,
        }
    }
    #[doc = "Checks if the value of the field is `MainMode`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR_A::MainMode
    }
    #[doc = "Checks if the value of the field is `LowPowerMode`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR_A::LowPowerMode
    }
}
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, LPR_A, O>;
impl<'a, const O: u8> LPR_W<'a, O> {
    #[doc = "Main Mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPR_A::MainMode)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPR_A::LowPowerMode)
    }
}
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - SRAM3 retention in Stop 2 mode"]
    #[inline(always)]
    pub fn rrstp(&self) -> RRSTP_R {
        RRSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    #[doc = "Bit 4 - SRAM3 retention in Stop 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrstp(&mut self) -> RRSTP_W<4> {
        RRSTP_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<9> {
        VOS_W::new(self)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<14> {
        LPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0200"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
