#[doc = "Register `RCC_PLL4CSGR` reader"]
pub struct R(crate::R<RCC_PLL4CSGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL4CSGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL4CSGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL4CSGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL4CSGR` writer"]
pub struct W(crate::W<RCC_PLL4CSGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL4CSGR_SPEC>;
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
impl From<crate::W<RCC_PLL4CSGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL4CSGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_PER` reader - MOD_PER"]
pub type MOD_PER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MOD_PER` writer - MOD_PER"]
pub type MOD_PER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_PLL4CSGR_SPEC, u16, u16, 13, O>;
#[doc = "Field `TPDFN_DIS` reader - TPDFN_DIS"]
pub type TPDFN_DIS_R = crate::BitReader<bool>;
#[doc = "Field `TPDFN_DIS` writer - TPDFN_DIS"]
pub type TPDFN_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL4CSGR_SPEC, bool, O>;
#[doc = "Field `RPDFN_DIS` reader - RPDFN_DIS"]
pub type RPDFN_DIS_R = crate::BitReader<bool>;
#[doc = "Field `RPDFN_DIS` writer - RPDFN_DIS"]
pub type RPDFN_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL4CSGR_SPEC, bool, O>;
#[doc = "Field `SSCG_MODE` reader - SSCG_MODE"]
pub type SSCG_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_MODE` writer - SSCG_MODE"]
pub type SSCG_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL4CSGR_SPEC, bool, O>;
#[doc = "Field `INC_STEP` reader - INC_STEP"]
pub type INC_STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INC_STEP` writer - INC_STEP"]
pub type INC_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_PLL4CSGR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&self) -> TPDFN_DIS_R {
        TPDFN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&self) -> RPDFN_DIS_R {
        RPDFN_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    #[must_use]
    pub fn mod_per(&mut self) -> MOD_PER_W<0> {
        MOD_PER_W::new(self)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn tpdfn_dis(&mut self) -> TPDFN_DIS_W<13> {
        TPDFN_DIS_W::new(self)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn rpdfn_dis(&mut self) -> RPDFN_DIS_W<14> {
        RPDFN_DIS_W::new(self)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W<15> {
        SSCG_MODE_W::new(self)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    #[must_use]
    pub fn inc_step(&mut self) -> INC_STEP_W<16> {
        INC_STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4csgr](index.html) module"]
pub struct RCC_PLL4CSGR_SPEC;
impl crate::RegisterSpec for RCC_PLL4CSGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll4csgr::R](R) reader structure"]
impl crate::Readable for RCC_PLL4CSGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll4csgr::W](W) writer structure"]
impl crate::Writable for RCC_PLL4CSGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_PLL4CSGR to value 0"]
impl crate::Resettable for RCC_PLL4CSGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
