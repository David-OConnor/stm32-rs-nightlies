#[doc = "Register `MACLTCR` reader"]
pub struct R(crate::R<MACLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACLTCR` writer"]
pub struct W(crate::W<MACLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLTCR_SPEC>;
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
impl From<crate::W<MACLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWT` reader - LPI TW Timer"]
pub type TWT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TWT` writer - LPI TW Timer"]
pub type TWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLTCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `LST` reader - LPI LS Timer"]
pub type LST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LST` writer - LPI LS Timer"]
pub type LST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLTCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<0> {
        TWT_W::new(self)
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<16> {
        LST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI timers control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macltcr](index.html) module"]
pub struct MACLTCR_SPEC;
impl crate::RegisterSpec for MACLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macltcr::R](R) reader structure"]
impl crate::Readable for MACLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macltcr::W](W) writer structure"]
impl crate::Writable for MACLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for MACLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03e8_0000;
}
