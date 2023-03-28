#[doc = "Register `D3CFGR` reader"]
pub struct R(crate::R<D3CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3CFGR` writer"]
pub struct W(crate::W<D3CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3CFGR_SPEC>;
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
impl From<crate::W<D3CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D3PPRE` reader - D3 domain APB4 prescaler"]
pub type D3PPRE_R = crate::FieldReader<u8, D3PPRE_A>;
#[doc = "D3 domain APB4 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D3PPRE_A {
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
impl From<D3PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: D3PPRE_A) -> Self {
        variant as _
    }
}
impl D3PPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D3PPRE_A> {
        match self.bits {
            0 => Some(D3PPRE_A::Div1),
            4 => Some(D3PPRE_A::Div2),
            5 => Some(D3PPRE_A::Div4),
            6 => Some(D3PPRE_A::Div8),
            7 => Some(D3PPRE_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D3PPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D3PPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D3PPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D3PPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D3PPRE_A::Div16
    }
}
#[doc = "Field `D3PPRE` writer - D3 domain APB4 prescaler"]
pub type D3PPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D3CFGR_SPEC, u8, D3PPRE_A, 3, O>;
impl<'a, const O: u8> D3PPRE_W<'a, O> {
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(D3PPRE_A::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(D3PPRE_A::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(D3PPRE_A::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(D3PPRE_A::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(D3PPRE_A::Div16)
    }
}
impl R {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&self) -> D3PPRE_R {
        D3PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d3ppre(&mut self) -> D3PPRE_W<4> {
        D3PPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Domain 3 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3cfgr](index.html) module"]
pub struct D3CFGR_SPEC;
impl crate::RegisterSpec for D3CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3cfgr::R](R) reader structure"]
impl crate::Readable for D3CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3cfgr::W](W) writer structure"]
impl crate::Writable for D3CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3CFGR to value 0"]
impl crate::Resettable for D3CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
