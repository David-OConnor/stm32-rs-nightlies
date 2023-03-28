#[doc = "Register `SHRR` reader"]
pub struct R(crate::R<SHRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHRR` writer"]
pub struct W(crate::W<SHRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHRR_SPEC>;
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
impl From<crate::W<SHRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TREFRESH1` reader - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN1=0."]
pub type TREFRESH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREFRESH1` writer - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN1=0."]
pub type TREFRESH1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHRR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TREFRESH2` reader - DAC channel2 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TREFRESH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREFRESH2` writer - DAC channel2 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TREFRESH2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHRR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN1=0."]
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAC channel2 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN1=0."]
    #[inline(always)]
    #[must_use]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<0> {
        TREFRESH1_W::new(self)
    }
    #[doc = "Bits 16:23 - DAC channel2 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    #[must_use]
    pub fn trefresh2(&mut self) -> TREFRESH2_W<16> {
        TREFRESH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Sample and Hold refresh time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shrr](index.html) module"]
pub struct SHRR_SPEC;
impl crate::RegisterSpec for SHRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shrr::R](R) reader structure"]
impl crate::Readable for SHRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shrr::W](W) writer structure"]
impl crate::Writable for SHRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHRR to value 0x0001_0001"]
impl crate::Resettable for SHRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
