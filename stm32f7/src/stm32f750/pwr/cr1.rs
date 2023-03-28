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
#[doc = "Field `LPDS` reader - Low-power deep sleep"]
pub type LPDS_R = crate::BitReader<bool>;
#[doc = "Field `LPDS` writer - Low-power deep sleep"]
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PDDS_R = crate::BitReader<PDDS_A>;
#[doc = "Power down deepsleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS_A {
    #[doc = "0: Enter Stop mode when the CPU enters deepsleep"]
    StopMode = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    StandbyMode = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::StopMode,
            true => PDDS_A::StandbyMode,
        }
    }
    #[doc = "Checks if the value of the field is `StopMode`"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS_A::StopMode
    }
    #[doc = "Checks if the value of the field is `StandbyMode`"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS_A::StandbyMode
    }
}
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PDDS_A, O>;
impl<'a, const O: u8> PDDS_W<'a, O> {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StopMode)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StandbyMode)
    }
}
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CSBF_R = crate::BitReader<bool>;
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `FPDS` reader - Flash power down in Stop mode"]
pub type FPDS_R = crate::BitReader<bool>;
#[doc = "Field `FPDS` writer - Flash power down in Stop mode"]
pub type FPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `LPUDS` reader - Low-power regulator in deepsleep under-drive mode"]
pub type LPUDS_R = crate::BitReader<bool>;
#[doc = "Field `LPUDS` writer - Low-power regulator in deepsleep under-drive mode"]
pub type LPUDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MRUDS` reader - Main regulator in deepsleep under-drive mode"]
pub type MRUDS_R = crate::BitReader<bool>;
#[doc = "Field `MRUDS` writer - Main regulator in deepsleep under-drive mode"]
pub type MRUDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ADCDC1` reader - ADCDC1"]
pub type ADCDC1_R = crate::BitReader<bool>;
#[doc = "Field `ADCDC1` writer - ADCDC1"]
pub type ADCDC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `VOS` reader - Regulator voltage scaling output selection"]
pub type VOS_R = crate::FieldReader<u8, VOS_A>;
#[doc = "Regulator voltage scaling output selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "1: Scale 3 mode"]
    Scale3 = 1,
    #[doc = "2: Scale 2 mode"]
    Scale2 = 2,
    #[doc = "3: Scale 1 mode (reset value)"]
    Scale1 = 3,
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
            1 => Some(VOS_A::Scale3),
            2 => Some(VOS_A::Scale2),
            3 => Some(VOS_A::Scale1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Scale3`"]
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == VOS_A::Scale3
    }
    #[doc = "Checks if the value of the field is `Scale2`"]
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == VOS_A::Scale2
    }
    #[doc = "Checks if the value of the field is `Scale1`"]
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == VOS_A::Scale1
    }
}
#[doc = "Field `VOS` writer - Regulator voltage scaling output selection"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, VOS_A, 2, O>;
impl<'a, const O: u8> VOS_W<'a, O> {
    #[doc = "Scale 3 mode"]
    #[inline(always)]
    pub fn scale3(self) -> &'a mut W {
        self.variant(VOS_A::Scale3)
    }
    #[doc = "Scale 2 mode"]
    #[inline(always)]
    pub fn scale2(self) -> &'a mut W {
        self.variant(VOS_A::Scale2)
    }
    #[doc = "Scale 1 mode (reset value)"]
    #[inline(always)]
    pub fn scale1(self) -> &'a mut W {
        self.variant(VOS_A::Scale1)
    }
}
#[doc = "Field `ODEN` reader - Over-drive enable"]
pub type ODEN_R = crate::BitReader<bool>;
#[doc = "Field `ODEN` writer - Over-drive enable"]
pub type ODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ODSWEN` reader - Over-drive switching enabled"]
pub type ODSWEN_R = crate::BitReader<bool>;
#[doc = "Field `ODSWEN` writer - Over-drive switching enabled"]
pub type ODSWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `UDEN` reader - Under-drive enable in stop mode"]
pub type UDEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDEN` writer - Under-drive enable in stop mode"]
pub type UDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-power regulator in deepsleep under-drive mode"]
    #[inline(always)]
    pub fn lpuds(&self) -> LPUDS_R {
        LPUDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Main regulator in deepsleep under-drive mode"]
    #[inline(always)]
    pub fn mruds(&self) -> MRUDS_R {
        MRUDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline(always)]
    pub fn adcdc1(&self) -> ADCDC1_R {
        ADCDC1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<0> {
        LPDS_W::new(self)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<1> {
        PDDS_W::new(self)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<3> {
        CSBF_W::new(self)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bit 9 - Flash power down in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<9> {
        FPDS_W::new(self)
    }
    #[doc = "Bit 10 - Low-power regulator in deepsleep under-drive mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuds(&mut self) -> LPUDS_W<10> {
        LPUDS_W::new(self)
    }
    #[doc = "Bit 11 - Main regulator in deepsleep under-drive mode"]
    #[inline(always)]
    #[must_use]
    pub fn mruds(&mut self) -> MRUDS_W<11> {
        MRUDS_W::new(self)
    }
    #[doc = "Bit 13 - ADCDC1"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc1(&mut self) -> ADCDC1_W<13> {
        ADCDC1_W::new(self)
    }
    #[doc = "Bits 14:15 - Regulator voltage scaling output selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<14> {
        VOS_W::new(self)
    }
    #[doc = "Bit 16 - Over-drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<16> {
        ODEN_W::new(self)
    }
    #[doc = "Bit 17 - Over-drive switching enabled"]
    #[inline(always)]
    #[must_use]
    pub fn odswen(&mut self) -> ODSWEN_W<17> {
        ODSWEN_W::new(self)
    }
    #[doc = "Bits 18:19 - Under-drive enable in stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn uden(&mut self) -> UDEN_W<18> {
        UDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
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
#[doc = "`reset()` method sets CR1 to value 0xc000"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000;
}
