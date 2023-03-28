#[doc = "Register `ETH_DMAC1CR` reader"]
pub struct R(crate::R<ETH_DMAC1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC1CR` writer"]
pub struct W(crate::W<ETH_DMAC1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC1CR_SPEC>;
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
impl From<crate::W<ETH_DMAC1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSS` reader - MSS"]
pub type MSS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MSS` writer - MSS"]
pub type MSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC1CR_SPEC, u16, u16, 14, O>;
#[doc = "Field `PBLX8` reader - PBLX8"]
pub type PBLX8_R = crate::BitReader<bool>;
#[doc = "Field `PBLX8` writer - PBLX8"]
pub type PBLX8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1CR_SPEC, bool, O>;
#[doc = "Field `DSL` reader - DSL"]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - DSL"]
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC1CR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<0> {
        MSS_W::new(self)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<16> {
        PBLX8_W::new(self)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<18> {
        DSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1cr](index.html) module"]
pub struct ETH_DMAC1CR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1cr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac1cr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1CR to value 0"]
impl crate::Resettable for ETH_DMAC1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
