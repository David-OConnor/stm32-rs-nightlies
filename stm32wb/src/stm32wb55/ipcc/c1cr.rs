#[doc = "Register `C1CR` reader"]
pub struct R(crate::R<C1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1CR` writer"]
pub struct W(crate::W<C1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1CR_SPEC>;
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
impl From<crate::W<C1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOIE` reader - processor 1 Receive channel occupied interrupt enable"]
pub type RXOIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOIE` writer - processor 1 Receive channel occupied interrupt enable"]
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
#[doc = "Field `TXFIE` reader - processor 1 Transmit channel free interrupt enable"]
pub type TXFIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIE` writer - processor 1 Transmit channel free interrupt enable"]
pub type TXFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - processor 1 Receive channel occupied interrupt enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel free interrupt enable"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - processor 1 Receive channel occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<0> {
        RXOIE_W::new(self)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel free interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<16> {
        TXFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1cr](index.html) module"]
pub struct C1CR_SPEC;
impl crate::RegisterSpec for C1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1cr::R](R) reader structure"]
impl crate::Readable for C1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1cr::W](W) writer structure"]
impl crate::Writable for C1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1CR to value 0"]
impl crate::Resettable for C1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
