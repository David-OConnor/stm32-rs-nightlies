#[doc = "Register `DAC_SR` reader"]
pub struct R(crate::R<DAC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SR` writer"]
pub struct W(crate::W<DAC_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SR_SPEC>;
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
impl From<crate::W<DAC_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAUDR1` reader - DMAUDR1"]
pub type DMAUDR1_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDR1` writer - DMAUDR1"]
pub type DMAUDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `CAL_FLAG1` reader - CAL_FLAG1"]
pub type CAL_FLAG1_R = crate::BitReader<bool>;
#[doc = "Field `BWST1` reader - BWST1"]
pub type BWST1_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDR2` reader - DMAUDR2"]
pub type DMAUDR2_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDR2` writer - DMAUDR2"]
pub type DMAUDR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SR_SPEC, bool, O>;
#[doc = "Field `CAL_FLAG2` reader - CAL_FLAG2"]
pub type CAL_FLAG2_R = crate::BitReader<bool>;
#[doc = "Field `BWST2` reader - BWST2"]
pub type BWST2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 13 - DMAUDR1"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAL_FLAG1"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BWST1"]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAUDR2"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CAL_FLAG2"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BWST2"]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DMAUDR1"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<13> {
        DMAUDR1_W::new(self)
    }
    #[doc = "Bit 29 - DMAUDR2"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<29> {
        DMAUDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sr](index.html) module"]
pub struct DAC_SR_SPEC;
impl crate::RegisterSpec for DAC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_sr::R](R) reader structure"]
impl crate::Readable for DAC_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_sr::W](W) writer structure"]
impl crate::Writable for DAC_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DAC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
