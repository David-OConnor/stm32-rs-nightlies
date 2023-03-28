#[doc = "Register `C2CR1` reader"]
pub struct R(crate::R<C2CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR1` writer"]
pub struct W(crate::W<C2CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR1_SPEC>;
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
impl From<crate::W<C2CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU2"]
pub type LPMS_R = crate::FieldReader<u8, LPMS_A>;
#[doc = "Low-power mode selection for CPU2\n\nValue on reset: 7"]
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
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU2"]
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2CR1_SPEC, u8, LPMS_A, 3, O>;
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
#[doc = "Field `FPDR` reader - Flash memory power down mode during LPRun for CPU2"]
pub type FPDR_R = crate::BitReader<FPDR_A>;
#[doc = "Flash memory power down mode during LPRun for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDR_A {
    #[doc = "0: Flash memory in Idle mode when system is in LPRun mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPRun mode"]
    PowerDown = 1,
}
impl From<FPDR_A> for bool {
    #[inline(always)]
    fn from(variant: FPDR_A) -> Self {
        variant as u8 != 0
    }
}
impl FPDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDR_A {
        match self.bits {
            false => FPDR_A::Idle,
            true => FPDR_A::PowerDown,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDR_A::Idle
    }
    #[doc = "Checks if the value of the field is `PowerDown`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDR_A::PowerDown
    }
}
#[doc = "Field `FPDR` writer - Flash memory power down mode during LPRun for CPU2"]
pub type FPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, FPDR_A, O>;
impl<'a, const O: u8> FPDR_W<'a, O> {
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDR_A::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDR_A::PowerDown)
    }
}
#[doc = "Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU2"]
pub type FPDS_R = crate::BitReader<FPDS_A>;
#[doc = "Flash memory power down mode during LPSleep for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS_A {
    #[doc = "0: Flash memory in Idle mode when system is in LPSleep mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPSleep mode"]
    PowerDown = 1,
}
impl From<FPDS_A> for bool {
    #[inline(always)]
    fn from(variant: FPDS_A) -> Self {
        variant as u8 != 0
    }
}
impl FPDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDS_A {
        match self.bits {
            false => FPDS_A::Idle,
            true => FPDS_A::PowerDown,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS_A::Idle
    }
    #[doc = "Checks if the value of the field is `PowerDown`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS_A::PowerDown
    }
}
#[doc = "Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU2"]
pub type FPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, FPDS_A, O>;
impl<'a, const O: u8> FPDS_W<'a, O> {
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDS_A::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDS_A::PowerDown)
    }
}
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<4> {
        FPDR_W::new(self)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<5> {
        FPDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power CPU2 control register 1 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr1](index.html) module"]
pub struct C2CR1_SPEC;
impl crate::RegisterSpec for C2CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr1::R](R) reader structure"]
impl crate::Readable for C2CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr1::W](W) writer structure"]
impl crate::Writable for C2CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2CR1 to value 0x07"]
impl crate::Resettable for C2CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
