#[doc = "Register `L2WHPCR` reader"]
pub struct R(crate::R<L2WHPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2WHPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2WHPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2WHPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2WHPCR` writer"]
pub struct W(crate::W<L2WHPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2WHPCR_SPEC>;
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
impl From<crate::W<L2WHPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2WHPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WHSTPOS` reader - Window Horizontal Start Position"]
pub type WHSTPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSTPOS` writer - Window Horizontal Start Position"]
pub type WHSTPOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, L2WHPCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `WHSPPOS` reader - Window Horizontal Stop Position"]
pub type WHSPPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WHSPPOS` writer - Window Horizontal Stop Position"]
pub type WHSPPOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, L2WHPCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<0> {
        WHSTPOS_W::new(self)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<16> {
        WHSPPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layerx Window Horizontal Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2whpcr](index.html) module"]
pub struct L2WHPCR_SPEC;
impl crate::RegisterSpec for L2WHPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2whpcr::R](R) reader structure"]
impl crate::Readable for L2WHPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2whpcr::W](W) writer structure"]
impl crate::Writable for L2WHPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2WHPCR to value 0"]
impl crate::Resettable for L2WHPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
