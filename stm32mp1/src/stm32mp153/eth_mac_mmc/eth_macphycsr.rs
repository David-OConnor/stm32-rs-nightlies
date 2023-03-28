#[doc = "Register `ETH_MACPHYCSR` reader"]
pub struct R(crate::R<ETH_MACPHYCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPHYCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPHYCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPHYCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPHYCSR` writer"]
pub struct W(crate::W<ETH_MACPHYCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPHYCSR_SPEC>;
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
impl From<crate::W<ETH_MACPHYCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPHYCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC` reader - TC"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - TC"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPHYCSR_SPEC, bool, O>;
#[doc = "Field `LUD` reader - LUD"]
pub type LUD_R = crate::BitReader<bool>;
#[doc = "Field `LUD` writer - LUD"]
pub type LUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPHYCSR_SPEC, bool, O>;
#[doc = "Field `LNKMOD` reader - LNKMOD"]
pub type LNKMOD_R = crate::BitReader<bool>;
#[doc = "Field `LNKSPEED` reader - LNKSPEED"]
pub type LNKSPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNKSTS` reader - LNKSTS"]
pub type LNKSTS_R = crate::BitReader<bool>;
#[doc = "Field `JABTO` reader - JABTO"]
pub type JABTO_R = crate::BitReader<bool>;
#[doc = "Field `FALSCARDET` reader - FALSCARDET"]
pub type FALSCARDET_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - LNKMOD"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LNKSPEED"]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - LNKSTS"]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - JABTO"]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FALSCARDET"]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<0> {
        TC_W::new(self)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    #[must_use]
    pub fn lud(&mut self) -> LUD_W<1> {
        LUD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macphycsr](index.html) module"]
pub struct ETH_MACPHYCSR_SPEC;
impl crate::RegisterSpec for ETH_MACPHYCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macphycsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPHYCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macphycsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPHYCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPHYCSR to value 0"]
impl crate::Resettable for ETH_MACPHYCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
