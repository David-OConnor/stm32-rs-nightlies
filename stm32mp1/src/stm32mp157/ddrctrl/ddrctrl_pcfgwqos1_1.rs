#[doc = "Register `DDRCTRL_PCFGWQOS1_1` reader"]
pub struct R(crate::R<DDRCTRL_PCFGWQOS1_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGWQOS1_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGWQOS1_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGWQOS1_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGWQOS1_1` writer"]
pub struct W(crate::W<DDRCTRL_PCFGWQOS1_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGWQOS1_1_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGWQOS1_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGWQOS1_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WQOS_MAP_TIMEOUT1` reader - WQOS_MAP_TIMEOUT1"]
pub type WQOS_MAP_TIMEOUT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WQOS_MAP_TIMEOUT1` writer - WQOS_MAP_TIMEOUT1"]
pub type WQOS_MAP_TIMEOUT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS1_1_SPEC, u16, u16, 11, O>;
#[doc = "Field `WQOS_MAP_TIMEOUT2` reader - WQOS_MAP_TIMEOUT2"]
pub type WQOS_MAP_TIMEOUT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WQOS_MAP_TIMEOUT2` writer - WQOS_MAP_TIMEOUT2"]
pub type WQOS_MAP_TIMEOUT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS1_1_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    pub fn wqos_map_timeout1(&self) -> WQOS_MAP_TIMEOUT1_R {
        WQOS_MAP_TIMEOUT1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    pub fn wqos_map_timeout2(&self) -> WQOS_MAP_TIMEOUT2_R {
        WQOS_MAP_TIMEOUT2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout1(&mut self) -> WQOS_MAP_TIMEOUT1_W<0> {
        WQOS_MAP_TIMEOUT1_W::new(self)
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout2(&mut self) -> WQOS_MAP_TIMEOUT2_W<16> {
        WQOS_MAP_TIMEOUT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 1 write Q0S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos1_1](index.html) module"]
pub struct DDRCTRL_PCFGWQOS1_1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGWQOS1_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgwqos1_1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS1_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos1_1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS1_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGWQOS1_1 to value 0"]
impl crate::Resettable for DDRCTRL_PCFGWQOS1_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}