#[doc = "Register `ETH_MACPCSR` reader"]
pub struct R(crate::R<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPCSR` writer"]
pub struct W(crate::W<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPCSR_SPEC>;
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
impl From<crate::W<ETH_MACPCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `MGKPKTEN` reader - MGKPKTEN"]
pub type MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPKTEN` writer - MGKPKTEN"]
pub type MGKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPKTEN` reader - RWKPKTEN"]
pub type RWKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `RWKPKTEN` writer - RWKPKTEN"]
pub type RWKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `MGKPRCVD` reader - MGKPRCVD"]
pub type MGKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `RWKPRCVD` reader - RWKPRCVD"]
pub type RWKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` reader - GLBLUCAST"]
pub type GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` writer - GLBLUCAST"]
pub type GLBLUCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPFE` reader - RWKPFE"]
pub type RWKPFE_R = crate::BitReader<bool>;
#[doc = "Field `RWKPFE` writer - RWKPFE"]
pub type RWKPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPTR` reader - RWKPTR"]
pub type RWKPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWKFILTRST` reader - RWKFILTRST"]
pub type RWKFILTRST_R = crate::BitReader<bool>;
#[doc = "Field `RWKFILTRST` writer - RWKFILTRST"]
pub type RWKFILTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MGKPRCVD"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWKPRCVD"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<1> {
        MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<2> {
        RWKPKTEN_W::new(self)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<10> {
        RWKPFE_W::new(self)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpcsr](index.html) module"]
pub struct ETH_MACPCSR_SPEC;
impl crate::RegisterSpec for ETH_MACPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macpcsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macpcsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPCSR to value 0"]
impl crate::Resettable for ETH_MACPCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
