#[doc = "Register `TZC_REGION_ID_ACCESS0` reader"]
pub struct R(crate::R<TZC_REGION_ID_ACCESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_ID_ACCESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_ID_ACCESS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_ID_ACCESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_REGION_ID_ACCESS0` writer"]
pub struct W(crate::W<TZC_REGION_ID_ACCESS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_ID_ACCESS0_SPEC>;
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
impl From<crate::W<TZC_REGION_ID_ACCESS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_ID_ACCESS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSAID_RD_EN` reader - NSAID_RD_EN"]
pub type NSAID_RD_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSAID_RD_EN` writer - NSAID_RD_EN"]
pub type NSAID_RD_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_ID_ACCESS0_SPEC, u16, u16, 16, O>;
#[doc = "Field `NSAID_WR_EN` reader - NSAID_WR_EN"]
pub type NSAID_WR_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSAID_WR_EN` writer - NSAID_WR_EN"]
pub type NSAID_WR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_ID_ACCESS0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - NSAID_RD_EN"]
    #[inline(always)]
    pub fn nsaid_rd_en(&self) -> NSAID_RD_EN_R {
        NSAID_RD_EN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NSAID_WR_EN"]
    #[inline(always)]
    pub fn nsaid_wr_en(&self) -> NSAID_WR_EN_R {
        NSAID_WR_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NSAID_RD_EN"]
    #[inline(always)]
    #[must_use]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W<0> {
        NSAID_RD_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - NSAID_WR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W<16> {
        NSAID_WR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region non-secure access based on NSAID.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_id_access0](index.html) module"]
pub struct TZC_REGION_ID_ACCESS0_SPEC;
impl crate::RegisterSpec for TZC_REGION_ID_ACCESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_id_access0::R](R) reader structure"]
impl crate::Readable for TZC_REGION_ID_ACCESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_region_id_access0::W](W) writer structure"]
impl crate::Writable for TZC_REGION_ID_ACCESS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZC_REGION_ID_ACCESS0 to value 0"]
impl crate::Resettable for TZC_REGION_ID_ACCESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
