#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR2` writer"]
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVM3IM` reader - PVM3IM"]
pub type PVM3IM_R = crate::BitReader<bool>;
#[doc = "Field `PVM3IM` writer - PVM3IM"]
pub type PVM3IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `PVDIM` reader - PVDIM"]
pub type PVDIM_R = crate::BitReader<bool>;
#[doc = "Field `PVDIM` writer - PVDIM"]
pub type PVDIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - PVM3IM"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - PVDIM"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - PVM3IM"]
    #[inline(always)]
    #[must_use]
    pub fn pvm3im(&mut self) -> PVM3IM_W<18> {
        PVM3IM_W::new(self)
    }
    #[doc = "Bit 20 - PVDIM"]
    #[inline(always)]
    #[must_use]
    pub fn pvdim(&mut self) -> PVDIM_W<20> {
        PVDIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU1 interrupt mask register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr2::W](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
