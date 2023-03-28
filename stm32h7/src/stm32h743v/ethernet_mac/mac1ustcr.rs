#[doc = "Register `MAC1USTCR` reader"]
pub struct R(crate::R<MAC1USTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC1USTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC1USTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC1USTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC1USTCR` writer"]
pub struct W(crate::W<MAC1USTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC1USTCR_SPEC>;
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
impl From<crate::W<MAC1USTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC1USTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIC_1US_CNTR` reader - 1 µs tick Counter"]
pub type TIC_1US_CNTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIC_1US_CNTR` writer - 1 µs tick Counter"]
pub type TIC_1US_CNTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC1USTCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - 1 µs tick Counter"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 1 µs tick Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<0> {
        TIC_1US_CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-microsecond-tick counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac1ustcr](index.html) module"]
pub struct MAC1USTCR_SPEC;
impl crate::RegisterSpec for MAC1USTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac1ustcr::R](R) reader structure"]
impl crate::Readable for MAC1USTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac1ustcr::W](W) writer structure"]
impl crate::Writable for MAC1USTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC1USTCR to value 0"]
impl crate::Resettable for MAC1USTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
