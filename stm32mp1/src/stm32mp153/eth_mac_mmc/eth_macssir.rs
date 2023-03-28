#[doc = "Register `ETH_MACSSIR` reader"]
pub struct R(crate::R<ETH_MACSSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSSIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSSIR` writer"]
pub struct W(crate::W<ETH_MACSSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSSIR_SPEC>;
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
impl From<crate::W<ETH_MACSSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSSIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNSINC` reader - SNSINC"]
pub type SNSINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNSINC` writer - SNSINC"]
pub type SNSINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSSIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SSINC` reader - SSINC"]
pub type SSINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSINC` writer - SSINC"]
pub type SSINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSSIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    #[must_use]
    pub fn snsinc(&mut self) -> SNSINC_W<8> {
        SNSINC_W::new(self)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<16> {
        SSINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macssir](index.html) module"]
pub struct ETH_MACSSIR_SPEC;
impl crate::RegisterSpec for ETH_MACSSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macssir::R](R) reader structure"]
impl crate::Readable for ETH_MACSSIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macssir::W](W) writer structure"]
impl crate::Writable for ETH_MACSSIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSSIR to value 0"]
impl crate::Resettable for ETH_MACSSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
