#[doc = "Register `hfpclkpllsel` reader"]
pub struct R(crate::R<HFPCLKPLLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPCLKPLLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPCLKPLLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPCLKPLLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfpclkpllsel` writer"]
pub struct W(crate::W<HFPCLKPLLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPCLKPLLSEL_SPEC>;
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
impl From<crate::W<HFPCLKPLLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPCLKPLLSEL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Periphery clock source register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpclkpllsel](index.html) module"]
pub struct HFPCLKPLLSEL_SPEC;
impl crate::RegisterSpec for HFPCLKPLLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfpclkpllsel::R](R) reader structure"]
impl crate::Readable for HFPCLKPLLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfpclkpllsel::W](W) writer structure"]
impl crate::Writable for HFPCLKPLLSEL_SPEC {
    type Writer = W;
}
