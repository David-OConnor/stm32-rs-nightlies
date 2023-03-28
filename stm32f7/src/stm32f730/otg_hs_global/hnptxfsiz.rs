#[doc = "Register `HNPTXFSIZ` reader"]
pub struct R(crate::R<HNPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HNPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HNPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HNPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HNPTXFSIZ` writer"]
pub struct W(crate::W<HNPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HNPTXFSIZ_SPEC>;
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
impl From<crate::W<HNPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HNPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPTXFSA` reader - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXFSA` writer - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HNPTXFSIZ_SPEC, u16, u16, 16, O>;
#[doc = "Field `NPTXFD` reader - Nonperiodic TxFIFO depth"]
pub type NPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXFD` writer - Nonperiodic TxFIFO depth"]
pub type NPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HNPTXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<0> {
        NPTXFSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<16> {
        NPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptxfsiz](index.html) module"]
pub struct HNPTXFSIZ_SPEC;
impl crate::RegisterSpec for HNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hnptxfsiz::R](R) reader structure"]
impl crate::Readable for HNPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hnptxfsiz::W](W) writer structure"]
impl crate::Writable for HNPTXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HNPTXFSIZ to value 0x0200"]
impl crate::Resettable for HNPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
