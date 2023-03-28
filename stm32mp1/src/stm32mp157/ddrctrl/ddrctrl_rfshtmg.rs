#[doc = "Register `DDRCTRL_RFSHTMG` reader"]
pub struct R(crate::R<DDRCTRL_RFSHTMG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_RFSHTMG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_RFSHTMG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_RFSHTMG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_RFSHTMG` writer"]
pub struct W(crate::W<DDRCTRL_RFSHTMG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_RFSHTMG_SPEC>;
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
impl From<crate::W<DDRCTRL_RFSHTMG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_RFSHTMG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RFC_MIN` reader - T_RFC_MIN"]
pub type T_RFC_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_RFC_MIN` writer - T_RFC_MIN"]
pub type T_RFC_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_RFSHTMG_SPEC, u16, u16, 10, O>;
#[doc = "Field `LPDDR3_TREFBW_EN` reader - LPDDR3_TREFBW_EN"]
pub type LPDDR3_TREFBW_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPDDR3_TREFBW_EN` writer - LPDDR3_TREFBW_EN"]
pub type LPDDR3_TREFBW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_RFSHTMG_SPEC, bool, O>;
#[doc = "Field `T_RFC_NOM_X1_X32` reader - T_RFC_NOM_X1_X32"]
pub type T_RFC_NOM_X1_X32_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_RFC_NOM_X1_X32` writer - T_RFC_NOM_X1_X32"]
pub type T_RFC_NOM_X1_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_RFSHTMG_SPEC, u16, u16, 12, O>;
#[doc = "Field `T_RFC_NOM_X1_SEL` reader - T_RFC_NOM_X1_SEL"]
pub type T_RFC_NOM_X1_SEL_R = crate::BitReader<bool>;
#[doc = "Field `T_RFC_NOM_X1_SEL` writer - T_RFC_NOM_X1_SEL"]
pub type T_RFC_NOM_X1_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_RFSHTMG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - T_RFC_MIN"]
    #[inline(always)]
    pub fn t_rfc_min(&self) -> T_RFC_MIN_R {
        T_RFC_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - LPDDR3_TREFBW_EN"]
    #[inline(always)]
    pub fn lpddr3_trefbw_en(&self) -> LPDDR3_TREFBW_EN_R {
        LPDDR3_TREFBW_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - T_RFC_NOM_X1_X32"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_x32(&self) -> T_RFC_NOM_X1_X32_R {
        T_RFC_NOM_X1_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - T_RFC_NOM_X1_SEL"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_sel(&self) -> T_RFC_NOM_X1_SEL_R {
        T_RFC_NOM_X1_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_RFC_MIN"]
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_min(&mut self) -> T_RFC_MIN_W<0> {
        T_RFC_MIN_W::new(self)
    }
    #[doc = "Bit 15 - LPDDR3_TREFBW_EN"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3_trefbw_en(&mut self) -> LPDDR3_TREFBW_EN_W<15> {
        LPDDR3_TREFBW_EN_W::new(self)
    }
    #[doc = "Bits 16:27 - T_RFC_NOM_X1_X32"]
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_nom_x1_x32(&mut self) -> T_RFC_NOM_X1_X32_W<16> {
        T_RFC_NOM_X1_X32_W::new(self)
    }
    #[doc = "Bit 31 - T_RFC_NOM_X1_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn t_rfc_nom_x1_sel(&mut self) -> T_RFC_NOM_X1_SEL_W<31> {
        T_RFC_NOM_X1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL refresh timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshtmg](index.html) module"]
pub struct DDRCTRL_RFSHTMG_SPEC;
impl crate::RegisterSpec for DDRCTRL_RFSHTMG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_rfshtmg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHTMG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshtmg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHTMG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHTMG to value 0x0062_008c"]
impl crate::Resettable for DDRCTRL_RFSHTMG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0062_008c;
}
