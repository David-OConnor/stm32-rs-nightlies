#[doc = "Register `PLLCKSELR` reader"]
pub struct R(crate::R<PLLCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCKSELR` writer"]
pub struct W(crate::W<PLLCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCKSELR_SPEC>;
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
impl From<crate::W<PLLCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - DIVMx and PLLs clock source selection"]
pub type PLLSRC_R = crate::FieldReader<u8, PLLSRC_A>;
#[doc = "DIVMx and PLLs clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: HSI selected as PLL clock"]
    Hsi = 0,
    #[doc = "1: CSI selected as PLL clock"]
    Csi = 1,
    #[doc = "2: HSE selected as PLL clock"]
    Hse = 2,
    #[doc = "3: No clock sent to DIVMx dividers and PLLs"]
    None = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::Hsi,
            1 => PLLSRC_A::Csi,
            2 => PLLSRC_A::Hse,
            3 => PLLSRC_A::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLLSRC_A::Csi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::Hse
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLSRC_A::None
    }
}
#[doc = "Field `PLLSRC` writer - DIVMx and PLLs clock source selection"]
pub type PLLSRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLCKSELR_SPEC, u8, PLLSRC_A, 2, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hsi)
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(PLLSRC_A::Csi)
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::Hse)
    }
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLSRC_A::None)
    }
}
#[doc = "Field `DIVM1` reader - Prescaler for PLL1"]
pub type DIVM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM1` writer - Prescaler for PLL1"]
pub type DIVM1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCKSELR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIVM2` reader - Prescaler for PLL2"]
pub type DIVM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM2` writer - Prescaler for PLL2"]
pub type DIVM2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCKSELR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIVM3` reader - Prescaler for PLL3"]
pub type DIVM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVM3` writer - Prescaler for PLL3"]
pub type DIVM3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLLCKSELR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    #[must_use]
    pub fn divm1(&mut self) -> DIVM1_W<4> {
        DIVM1_W::new(self)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    #[must_use]
    pub fn divm2(&mut self) -> DIVM2_W<12> {
        DIVM2_W::new(self)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    #[must_use]
    pub fn divm3(&mut self) -> DIVM3_W<20> {
        DIVM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLLs Clock Source Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllckselr](index.html) module"]
pub struct PLLCKSELR_SPEC;
impl crate::RegisterSpec for PLLCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllckselr::R](R) reader structure"]
impl crate::Readable for PLLCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllckselr::W](W) writer structure"]
impl crate::Writable for PLLCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCKSELR to value 0x0202_0200"]
impl crate::Resettable for PLLCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_0200;
}
