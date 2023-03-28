#[doc = "Register `DDRCTRL_PCFGQOS1_0` reader"]
pub struct R(crate::R<DDRCTRL_PCFGQOS1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGQOS1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGQOS1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGQOS1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGQOS1_0` writer"]
pub struct W(crate::W<DDRCTRL_PCFGQOS1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGQOS1_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGQOS1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGQOS1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RQOS_MAP_TIMEOUTB` reader - RQOS_MAP_TIMEOUTB"]
pub type RQOS_MAP_TIMEOUTB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RQOS_MAP_TIMEOUTB` writer - RQOS_MAP_TIMEOUTB"]
pub type RQOS_MAP_TIMEOUTB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGQOS1_0_SPEC, u16, u16, 11, O>;
#[doc = "Field `RQOS_MAP_TIMEOUTR` reader - RQOS_MAP_TIMEOUTR"]
pub type RQOS_MAP_TIMEOUTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RQOS_MAP_TIMEOUTR` writer - RQOS_MAP_TIMEOUTR"]
pub type RQOS_MAP_TIMEOUTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGQOS1_0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - RQOS_MAP_TIMEOUTB"]
    #[inline(always)]
    pub fn rqos_map_timeoutb(&self) -> RQOS_MAP_TIMEOUTB_R {
        RQOS_MAP_TIMEOUTB_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - RQOS_MAP_TIMEOUTR"]
    #[inline(always)]
    pub fn rqos_map_timeoutr(&self) -> RQOS_MAP_TIMEOUTR_R {
        RQOS_MAP_TIMEOUTR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - RQOS_MAP_TIMEOUTB"]
    #[inline(always)]
    #[must_use]
    pub fn rqos_map_timeoutb(&mut self) -> RQOS_MAP_TIMEOUTB_W<0> {
        RQOS_MAP_TIMEOUTB_W::new(self)
    }
    #[doc = "Bits 16:26 - RQOS_MAP_TIMEOUTR"]
    #[inline(always)]
    #[must_use]
    pub fn rqos_map_timeoutr(&mut self) -> RQOS_MAP_TIMEOUTR_W<16> {
        RQOS_MAP_TIMEOUTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 0 read Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos1_0](index.html) module"]
pub struct DDRCTRL_PCFGQOS1_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGQOS1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgqos1_0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos1_0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS1_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGQOS1_0 to value 0"]
impl crate::Resettable for DDRCTRL_PCFGQOS1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
