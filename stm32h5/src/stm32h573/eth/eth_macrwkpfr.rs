#[doc = "Register `ETH_MACRWKPFR` reader"]
pub struct R(crate::R<ETH_MACRWKPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRWKPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRWKPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRWKPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRWKPFR` writer"]
pub struct W(crate::W<ETH_MACRWKPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRWKPFR_SPEC>;
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
impl From<crate::W<ETH_MACRWKPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRWKPFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACRWKPFR` reader - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence."]
pub type MACRWKPFR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MACRWKPFR` writer - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence."]
pub type MACRWKPFR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACRWKPFR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence."]
    #[inline(always)]
    pub fn macrwkpfr(&self) -> MACRWKPFR_R {
        MACRWKPFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence."]
    #[inline(always)]
    #[must_use]
    pub fn macrwkpfr(&mut self) -> MACRWKPFR_W<0> {
        MACRWKPFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remote wakeup packet filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrwkpfr](index.html) module"]
pub struct ETH_MACRWKPFR_SPEC;
impl crate::RegisterSpec for ETH_MACRWKPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrwkpfr::R](R) reader structure"]
impl crate::Readable for ETH_MACRWKPFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrwkpfr::W](W) writer structure"]
impl crate::Writable for ETH_MACRWKPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACRWKPFR to value 0"]
impl crate::Resettable for ETH_MACRWKPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
