#[doc = "Register `D3PCR1H` reader"]
pub struct R(crate::R<D3PCR1H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR1H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR1H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR1H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR1H` writer"]
pub struct W(crate::W<D3PCR1H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR1H_SPEC>;
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
impl From<crate::W<D3PCR1H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR1H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_R = crate::FieldReader<u8, PCS19_A>;
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS19_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS19_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS19_A) -> Self {
        variant as _
    }
}
impl PCS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS19_A {
        match self.bits {
            0 => PCS19_A::DmaCh6,
            1 => PCS19_A::DmaCh7,
            2 => PCS19_A::Lptim4,
            3 => PCS19_A::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DmaCh6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS19_A::DmaCh6
    }
    #[doc = "Checks if the value of the field is `DmaCh7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS19_A::DmaCh7
    }
    #[doc = "Checks if the value of the field is `Lptim4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS19_A::Lptim4
    }
    #[doc = "Checks if the value of the field is `Lptim5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS19_A::Lptim5
    }
}
#[doc = "Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D3PCR1H_SPEC, u8, PCS19_A, 2, O>;
impl<'a, const O: u8> PCS19_W<'a, O> {
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS19_A::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS19_A::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS19_A::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS19_A::Lptim5)
    }
}
#[doc = "Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS20_R;
#[doc = "Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS21_R;
#[doc = "Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS25_R;
#[doc = "Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS20_W;
#[doc = "Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS21_W;
#[doc = "Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS25_W;
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs19(&mut self) -> PCS19_W<6> {
        PCS19_W::new(self)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs20(&mut self) -> PCS20_W<8> {
        PCS20_W::new(self)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs21(&mut self) -> PCS21_W<10> {
        PCS21_W::new(self)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs25(&mut self) -> PCS25_W<18> {
        PCS25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1h](index.html) module"]
pub struct D3PCR1H_SPEC;
impl crate::RegisterSpec for D3PCR1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr1h::R](R) reader structure"]
impl crate::Readable for D3PCR1H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr1h::W](W) writer structure"]
impl crate::Writable for D3PCR1H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR1H to value 0"]
impl crate::Resettable for D3PCR1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
