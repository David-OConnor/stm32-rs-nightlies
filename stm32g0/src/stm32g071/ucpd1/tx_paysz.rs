#[doc = "Register `TX_PAYSZ` reader"]
pub struct R(crate::R<TX_PAYSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PAYSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PAYSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PAYSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PAYSZ` writer"]
pub struct W(crate::W<TX_PAYSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PAYSZ_SPEC>;
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
impl From<crate::W<TX_PAYSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PAYSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPAYSZ` reader - TXPAYSZ"]
pub type TXPAYSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXPAYSZ` writer - TXPAYSZ"]
pub type TXPAYSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_PAYSZ_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    #[must_use]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W<0> {
        TXPAYSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Tx Paysize Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_paysz](index.html) module"]
pub struct TX_PAYSZ_SPEC;
impl crate::RegisterSpec for TX_PAYSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_paysz::R](R) reader structure"]
impl crate::Readable for TX_PAYSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_paysz::W](W) writer structure"]
impl crate::Writable for TX_PAYSZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PAYSZ to value 0"]
impl crate::Resettable for TX_PAYSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
