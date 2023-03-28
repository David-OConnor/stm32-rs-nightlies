#[doc = "Register `DDRCTRL_PCTRL_0` reader"]
pub struct R(crate::R<DDRCTRL_PCTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCTRL_0` writer"]
pub struct W(crate::W<DDRCTRL_PCTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCTRL_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT_EN` reader - PORT_EN"]
pub type PORT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PORT_EN` writer - PORT_EN"]
pub type PORT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PCTRL_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    pub fn port_en(&self) -> PORT_EN_R {
        PORT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT_EN"]
    #[inline(always)]
    #[must_use]
    pub fn port_en(&mut self) -> PORT_EN_W<0> {
        PORT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pctrl_0](index.html) module"]
pub struct DDRCTRL_PCTRL_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pctrl_0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pctrl_0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCTRL_0 to value 0"]
impl crate::Resettable for DDRCTRL_PCTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
