#[doc = "Register `DDRCTRL_DRAMTMG6` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG6` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG6_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_CKCSX` reader - T_CKCSX"]
pub type T_CKCSX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKCSX` writer - T_CKCSX"]
pub type T_CKCSX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG6_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CKDPDX` reader - T_CKDPDX"]
pub type T_CKDPDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKDPDX` writer - T_CKDPDX"]
pub type T_CKDPDX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG6_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_CKDPDE` reader - T_CKDPDE"]
pub type T_CKDPDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKDPDE` writer - T_CKDPDE"]
pub type T_CKDPDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG6_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    pub fn t_ckcsx(&self) -> T_CKCSX_R {
        T_CKCSX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    pub fn t_ckdpdx(&self) -> T_CKDPDX_R {
        T_CKDPDX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    pub fn t_ckdpde(&self) -> T_CKDPDE_R {
        T_CKDPDE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKCSX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckcsx(&mut self) -> T_CKCSX_W<0> {
        T_CKCSX_W::new(self)
    }
    #[doc = "Bits 16:19 - T_CKDPDX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckdpdx(&mut self) -> T_CKDPDX_W<16> {
        T_CKDPDX_W::new(self)
    }
    #[doc = "Bits 24:27 - T_CKDPDE"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckdpde(&mut self) -> T_CKDPDE_W<24> {
        T_CKDPDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg6](index.html) module"]
pub struct DDRCTRL_DRAMTMG6_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg6::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg6::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG6 to value 0x0202_0005"]
impl crate::Resettable for DDRCTRL_DRAMTMG6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_0005;
}
