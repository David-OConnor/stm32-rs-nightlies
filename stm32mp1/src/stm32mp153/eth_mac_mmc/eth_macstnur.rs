#[doc = "Register `ETH_MACSTNUR` reader"]
pub struct R(crate::R<ETH_MACSTNUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTNUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTNUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTNUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSTNUR` writer"]
pub struct W(crate::W<ETH_MACSTNUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSTNUR_SPEC>;
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
impl From<crate::W<ETH_MACSTNUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSTNUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSS` reader - TSSS"]
pub type TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSSS` writer - TSSS"]
pub type TSSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSTNUR_SPEC, u32, u32, 31, O>;
#[doc = "Field `ADDSUB` reader - ADDSUB"]
pub type ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUB` writer - ADDSUB"]
pub type ADDSUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACSTNUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<0> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<31> {
        ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstnur](index.html) module"]
pub struct ETH_MACSTNUR_SPEC;
impl crate::RegisterSpec for ETH_MACSTNUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstnur::R](R) reader structure"]
impl crate::Readable for ETH_MACSTNUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macstnur::W](W) writer structure"]
impl crate::Writable for ETH_MACSTNUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSTNUR to value 0"]
impl crate::Resettable for ETH_MACSTNUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
