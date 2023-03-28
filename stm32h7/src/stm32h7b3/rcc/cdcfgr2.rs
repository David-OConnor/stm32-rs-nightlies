#[doc = "Register `CDCFGR2` reader"]
pub struct R(crate::R<CDCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCFGR2` writer"]
pub struct W(crate::W<CDCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCFGR2_SPEC>;
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
impl From<crate::W<CDCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDPPRE1` reader - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type CDPPRE1_R = crate::FieldReader<u8, CDPPRE1_A>;
#[doc = "CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDPPRE1_A {
    #[doc = "0: rcc_hclk not divided"]
    Div1 = 0,
    #[doc = "4: rcc_hclk divided by 2"]
    Div2 = 4,
    #[doc = "5: rcc_hclk divided by 4"]
    Div4 = 5,
    #[doc = "6: rcc_hclk divided by 8"]
    Div8 = 6,
    #[doc = "7: rcc_hclk divided by 16"]
    Div16 = 7,
}
impl From<CDPPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: CDPPRE1_A) -> Self {
        variant as _
    }
}
impl CDPPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDPPRE1_A> {
        match self.bits {
            0 => Some(CDPPRE1_A::Div1),
            4 => Some(CDPPRE1_A::Div2),
            5 => Some(CDPPRE1_A::Div4),
            6 => Some(CDPPRE1_A::Div8),
            7 => Some(CDPPRE1_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CDPPRE1_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CDPPRE1_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CDPPRE1_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CDPPRE1_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CDPPRE1_A::Div16
    }
}
#[doc = "Field `CDPPRE1` writer - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type CDPPRE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCFGR2_SPEC, u8, CDPPRE1_A, 3, O>;
impl<'a, const O: u8> CDPPRE1_W<'a, O> {
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CDPPRE1_A::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CDPPRE1_A::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CDPPRE1_A::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CDPPRE1_A::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CDPPRE1_A::Div16)
    }
}
#[doc = "Field `CDPPRE2` reader - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
pub use CDPPRE1_R as CDPPRE2_R;
#[doc = "Field `CDPPRE2` writer - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
pub use CDPPRE1_W as CDPPRE2_W;
impl R {
    #[doc = "Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn cdppre1(&self) -> CDPPRE1_R {
        CDPPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn cdppre2(&self) -> CDPPRE2_R {
        CDPPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre1(&mut self) -> CDPPRE1_W<4> {
        CDPPRE1_W::new(self)
    }
    #[doc = "Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre2(&mut self) -> CDPPRE2_W<8> {
        CDPPRE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdcfgr2](index.html) module"]
pub struct CDCFGR2_SPEC;
impl crate::RegisterSpec for CDCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdcfgr2::R](R) reader structure"]
impl crate::Readable for CDCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdcfgr2::W](W) writer structure"]
impl crate::Writable for CDCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCFGR2 to value 0"]
impl crate::Resettable for CDCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
