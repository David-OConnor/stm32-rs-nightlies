#[doc = "Register `CKDIV` reader"]
pub struct R(crate::R<CKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKDIV` writer"]
pub struct W(crate::W<CKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKDIV_SPEC>;
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
impl From<crate::W<CKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKDIV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<0> {
        PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN CFG clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckdiv](index.html) module"]
pub struct CKDIV_SPEC;
impl crate::RegisterSpec for CKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckdiv::R](R) reader structure"]
impl crate::Readable for CKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckdiv::W](W) writer structure"]
impl crate::Writable for CKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKDIV to value 0"]
impl crate::Resettable for CKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
