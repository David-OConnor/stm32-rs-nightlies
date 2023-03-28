#[doc = "Register `DDRCTRL_DRAMTMG15` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG15` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG15_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_STAB_X32` reader - T_STAB_X32"]
pub type T_STAB_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_STAB_X32` writer - T_STAB_X32"]
pub type T_STAB_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DRAMTMG15_SPEC, u8, u8, 8, O>;
#[doc = "Field `EN_DFI_LP_T_STAB` reader - EN_DFI_LP_T_STAB"]
pub type EN_DFI_LP_T_STAB_R = crate::BitReader<bool>;
#[doc = "Field `EN_DFI_LP_T_STAB` writer - EN_DFI_LP_T_STAB"]
pub type EN_DFI_LP_T_STAB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DRAMTMG15_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&self) -> T_STAB_X32_R {
        T_STAB_X32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&self) -> EN_DFI_LP_T_STAB_R {
        EN_DFI_LP_T_STAB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    #[must_use]
    pub fn t_stab_x32(&mut self) -> T_STAB_X32_W<0> {
        T_STAB_X32_W::new(self)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    #[must_use]
    pub fn en_dfi_lp_t_stab(&mut self) -> EN_DFI_LP_T_STAB_W<31> {
        EN_DFI_LP_T_STAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg15](index.html) module"]
pub struct DDRCTRL_DRAMTMG15_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg15::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg15::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG15 to value 0"]
impl crate::Resettable for DDRCTRL_DRAMTMG15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
