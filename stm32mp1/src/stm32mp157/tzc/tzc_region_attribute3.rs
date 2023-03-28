#[doc = "Register `TZC_REGION_ATTRIBUTE3` reader"]
pub struct R(crate::R<TZC_REGION_ATTRIBUTE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_ATTRIBUTE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_ATTRIBUTE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_ATTRIBUTE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_REGION_ATTRIBUTE3` writer"]
pub struct W(crate::W<TZC_REGION_ATTRIBUTE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_ATTRIBUTE3_SPEC>;
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
impl From<crate::W<TZC_REGION_ATTRIBUTE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_ATTRIBUTE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_EN` reader - FILTER_EN"]
pub type FILTER_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER_EN` writer - FILTER_EN"]
pub type FILTER_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_ATTRIBUTE3_SPEC, u8, u8, 2, O>;
#[doc = "Field `S_RD_EN` reader - S_RD_EN"]
pub type S_RD_EN_R = crate::BitReader<bool>;
#[doc = "Field `S_RD_EN` writer - S_RD_EN"]
pub type S_RD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_REGION_ATTRIBUTE3_SPEC, bool, O>;
#[doc = "Field `S_WR_EN` reader - S_WR_EN"]
pub type S_WR_EN_R = crate::BitReader<bool>;
#[doc = "Field `S_WR_EN` writer - S_WR_EN"]
pub type S_WR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_REGION_ATTRIBUTE3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&self) -> S_RD_EN_R {
        S_RD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&self) -> S_WR_EN_R {
        S_WR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<0> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    #[must_use]
    pub fn s_rd_en(&mut self) -> S_RD_EN_W<30> {
        S_RD_EN_W::new(self)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn s_wr_en(&mut self) -> S_WR_EN_W<31> {
        S_WR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region x attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_attribute3](index.html) module"]
pub struct TZC_REGION_ATTRIBUTE3_SPEC;
impl crate::RegisterSpec for TZC_REGION_ATTRIBUTE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_attribute3::R](R) reader structure"]
impl crate::Readable for TZC_REGION_ATTRIBUTE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_region_attribute3::W](W) writer structure"]
impl crate::Writable for TZC_REGION_ATTRIBUTE3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZC_REGION_ATTRIBUTE3 to value 0"]
impl crate::Resettable for TZC_REGION_ATTRIBUTE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
