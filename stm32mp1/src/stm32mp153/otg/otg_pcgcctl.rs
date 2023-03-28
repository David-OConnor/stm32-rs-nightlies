#[doc = "Register `OTG_PCGCCTL` reader"]
pub struct R(crate::R<OTG_PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_PCGCCTL` writer"]
pub struct W(crate::W<OTG_PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_PCGCCTL_SPEC>;
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
impl From<crate::W<OTG_PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPPCLK` reader - STPPCLK"]
pub type STPPCLK_R = crate::BitReader<bool>;
#[doc = "Field `STPPCLK` writer - STPPCLK"]
pub type STPPCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_PCGCCTL_SPEC, bool, O>;
#[doc = "Field `GATEHCLK` reader - GATEHCLK"]
pub type GATEHCLK_R = crate::BitReader<bool>;
#[doc = "Field `GATEHCLK` writer - GATEHCLK"]
pub type GATEHCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PHYSUSP` reader - PHYSUSP"]
pub type PHYSUSP_R = crate::BitReader<bool>;
#[doc = "Field `ENL1GTG` reader - ENL1GTG"]
pub type ENL1GTG_R = crate::BitReader<bool>;
#[doc = "Field `ENL1GTG` writer - ENL1GTG"]
pub type ENL1GTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PHYSLEEP` reader - PHYSLEEP"]
pub type PHYSLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` reader - SUSP"]
pub type SUSP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHYSUSP"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYSLEEP"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<0> {
        STPPCLK_W::new(self)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<1> {
        GATEHCLK_W::new(self)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    #[must_use]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<5> {
        ENL1GTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is available in host and device modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_pcgcctl](index.html) module"]
pub struct OTG_PCGCCTL_SPEC;
impl crate::RegisterSpec for OTG_PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_pcgcctl::R](R) reader structure"]
impl crate::Readable for OTG_PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_pcgcctl::W](W) writer structure"]
impl crate::Writable for OTG_PCGCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for OTG_PCGCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x200b_8000;
}
