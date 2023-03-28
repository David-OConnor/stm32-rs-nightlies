#[doc = "Register `TX_ORDSET` reader"]
pub struct R(crate::R<TX_ORDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ORDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ORDSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ORDSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_ORDSET` writer"]
pub struct W(crate::W<TX_ORDSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ORDSET_SPEC>;
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
impl From<crate::W<TX_ORDSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ORDSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXORDSET` reader - TXORDSET"]
pub type TXORDSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXORDSET` writer - TXORDSET"]
pub type TXORDSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_ORDSET_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<0> {
        TXORDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Tx Ordered Set Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ordset](index.html) module"]
pub struct TX_ORDSET_SPEC;
impl crate::RegisterSpec for TX_ORDSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ordset::R](R) reader structure"]
impl crate::Readable for TX_ORDSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ordset::W](W) writer structure"]
impl crate::Writable for TX_ORDSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_ORDSET to value 0"]
impl crate::Resettable for TX_ORDSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
