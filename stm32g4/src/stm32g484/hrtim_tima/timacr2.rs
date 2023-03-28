#[doc = "Register `TIMACR2` reader"]
pub struct R(crate::R<TIMACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMACR2` writer"]
pub struct W(crate::W<TIMACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMACR2_SPEC>;
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
impl From<crate::W<TIMACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDE` reader - Dual Channel DAC trigger enable"]
pub type DCDE_R = crate::BitReader<bool>;
#[doc = "Field `DCDE` writer - Dual Channel DAC trigger enable"]
pub type DCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `DCDS` reader - Dual Channel DAC Step trigger"]
pub type DCDS_R = crate::BitReader<bool>;
#[doc = "Field `DCDS` writer - Dual Channel DAC Step trigger"]
pub type DCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `DCDR` reader - Dual Channel DAC Reset trigger"]
pub type DCDR_R = crate::BitReader<bool>;
#[doc = "Field `DCDR` writer - Dual Channel DAC Reset trigger"]
pub type DCDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `UDM` reader - Up-Down Mode"]
pub type UDM_R = crate::BitReader<bool>;
#[doc = "Field `UDM` writer - Up-Down Mode"]
pub type UDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `ROM` reader - Roll-Over Mode"]
pub type ROM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROM` writer - Roll-Over Mode"]
pub type ROM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMACR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUTROM` reader - Output Roll-Over Mode"]
pub type OUTROM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTROM` writer - Output Roll-Over Mode"]
pub type OUTROM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMACR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADROM` reader - ADC Roll-Over Mode"]
pub type ADROM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADROM` writer - ADC Roll-Over Mode"]
pub type ADROM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMACR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `BMROM` reader - Burst Mode Roll-Over Mode"]
pub type BMROM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMROM` writer - Burst Mode Roll-Over Mode"]
pub type BMROM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMACR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `FEROM` reader - Fault and Event Roll-Over Mode"]
pub type FEROM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEROM` writer - Fault and Event Roll-Over Mode"]
pub type FEROM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMACR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `GTCMP1` reader - Greater than Compare 1 PWM mode"]
pub type GTCMP1_R = crate::BitReader<bool>;
#[doc = "Field `GTCMP1` writer - Greater than Compare 1 PWM mode"]
pub type GTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `GTCMP3` reader - Greater than Compare 3 PWM mode"]
pub type GTCMP3_R = crate::BitReader<bool>;
#[doc = "Field `GTCMP3` writer - Greater than Compare 3 PWM mode"]
pub type GTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
#[doc = "Field `TRGHLF` reader - Triggered-half mode"]
pub type TRGHLF_R = crate::BitReader<bool>;
#[doc = "Field `TRGHLF` writer - Triggered-half mode"]
pub type TRGHLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMACR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    pub fn dcde(&self) -> DCDE_R {
        DCDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    pub fn udm(&self) -> UDM_R {
        UDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    pub fn outrom(&self) -> OUTROM_R {
        OUTROM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    pub fn adrom(&self) -> ADROM_R {
        ADROM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    pub fn bmrom(&self) -> BMROM_R {
        BMROM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    pub fn ferom(&self) -> FEROM_R {
        FEROM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    pub fn gtcmp1(&self) -> GTCMP1_R {
        GTCMP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    pub fn gtcmp3(&self) -> GTCMP3_R {
        GTCMP3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    pub fn trghlf(&self) -> TRGHLF_R {
        TRGHLF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcde(&mut self) -> DCDE_W<0> {
        DCDE_W::new(self)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcds(&mut self) -> DCDS_W<1> {
        DCDS_W::new(self)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcdr(&mut self) -> DCDR_W<2> {
        DCDR_W::new(self)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    #[must_use]
    pub fn udm(&mut self) -> UDM_W<4> {
        UDM_W::new(self)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<6> {
        ROM_W::new(self)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn outrom(&mut self) -> OUTROM_W<8> {
        OUTROM_W::new(self)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adrom(&mut self) -> ADROM_W<10> {
        ADROM_W::new(self)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmrom(&mut self) -> BMROM_W<12> {
        BMROM_W::new(self)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ferom(&mut self) -> FEROM_W<14> {
        FEROM_W::new(self)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp1(&mut self) -> GTCMP1_W<16> {
        GTCMP1_W::new(self)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp3(&mut self) -> GTCMP3_W<17> {
        GTCMP3_W::new(self)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    #[must_use]
    pub fn trghlf(&mut self) -> TRGHLF_W<20> {
        TRGHLF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timacr2](index.html) module"]
pub struct TIMACR2_SPEC;
impl crate::RegisterSpec for TIMACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timacr2::R](R) reader structure"]
impl crate::Readable for TIMACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timacr2::W](W) writer structure"]
impl crate::Writable for TIMACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMACR2 to value 0"]
impl crate::Resettable for TIMACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
