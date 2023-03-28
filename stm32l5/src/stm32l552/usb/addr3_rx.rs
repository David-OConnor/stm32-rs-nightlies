#[doc = "Register `ADDR3_RX` reader"]
pub struct R(crate::R<ADDR3_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR3_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR3_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR3_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR3_RX` writer"]
pub struct W(crate::W<ADDR3_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR3_RX_SPEC>;
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
impl From<crate::W<ADDR3_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR3_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR3_RX` reader - Reception buffer address"]
pub type ADDR3_RX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR3_RX` writer - Reception buffer address"]
pub type ADDR3_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADDR3_RX_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr3_rx(&self) -> ADDR3_RX_R {
        ADDR3_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr3_rx(&mut self) -> ADDR3_RX_W<1> {
        ADDR3_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr3_rx](index.html) module"]
pub struct ADDR3_RX_SPEC;
impl crate::RegisterSpec for ADDR3_RX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addr3_rx::R](R) reader structure"]
impl crate::Readable for ADDR3_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr3_rx::W](W) writer structure"]
impl crate::Writable for ADDR3_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR3_RX to value 0"]
impl crate::Resettable for ADDR3_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}